/*!

Demonstrates compile time registration, potentially across crate boundaries.

## Design

- Each plugin describes its own validation requirements.
- Plugins can be enabled/disabled.
- Modules requiring a particular plugin must enforce this requirement in their `init(context: &mut Context)` function.
- Thus, model authors must perform module configuration prior to `init()` being called if a module allows
  disabling/enabling a plugin.
- The `init()` functions are called in arbitrary order in the constructor of `Context`--although we could have a
  dependency mechanism if we want.
- In this design, it is the `init` functions that are statically registered, but this is an implementation detail.

### Application to Entity Properties

A module "owns" the properties declared within it by _convention_.

- Opt in/out mechanism is determined by the module itself.
    - plugins can be unconditionally enabled by the module
    - can be opt-out (if module supports it)
    - can be opt-in (if module supports it)
- Configuration errors occur at time of context creation under the assumption that `init()` validates configuration.
- A variety of value initialization strategies are supported in this paradigm: entity creation time, entity access time,
  entity write time (though this has issues, as we discussed).

#### Q: If we require every module to have an `init()`, what's the point of statically defining the plugins?

A: The `init()` functions can be declared anywhere in the codebase, including across crate boundaries, and called
automatically in the constructor for `Context`. In the implementation below, we use the `Property` type, which has an
`init` method.


Open questions:
- What happens with naming conflicts?

## Implementation Mechanism

Uses the Distributed Slice from the [`linkme` crate](https://github.com/dtolnay/linkme) to create a registry of "plugins" (standing in for person properties,
data plugins, etc.).

*/


use linkme::distributed_slice;

/// There are a million ways to do this. In this simple example we just have a `Plugin` type. This array is GLOBAL and 
/// determined at compile time.
#[distributed_slice]
pub static PLUGINS: [Plugin];

/// In this example, the `Plugin` type holds configuration that can affect what happens when `init` is called. The
/// constructor of `Context` iterates over all plugins in the static "Distributed Slice", calling `plugin.init(context)`
/// with each `plugin`.
pub struct Plugin {
    pub name: &'static str,
    pub description: &'static str,
    /// Required means must have a value for every entity
    pub required: bool,
    /// Enabled means this property is instantiated in the `Context`
    pub enabled: bool,
    /// The initializer knows how to compute the first value assigned to an entity
    pub initializer: fn(&mut Context, person_id: usize),
    //... etc.
    
    pub constructor: fn(&mut Context)
}

impl Plugin {
    pub fn init(&self, context: &mut Context) {
        // Maybe there is a PluginInstance type that gets stored in `context`, or maybe
        // there is an api for configuring entity properties that this method interacts with,
        // or....
        //
        // This is also where the database of property metadata would be initialized: TypeId->metadata
        (self.constructor)(context);
    }
}

#[derive(Default)]
pub struct Context {
    pub plugins: Vec<&'static str>
}

impl Context {
    pub fn new() -> Self {
        let mut context = Context::default();
        for plugin in PLUGINS.iter() {
            plugin.init(&mut context);
        }

        // If we validate dependency constraints, we would do it here, because the iteration order is nondeterministic.

        context
    }

    pub fn register_plugin(&mut self, plugin: &Plugin) {
        self.plugins.push(plugin.name);
    }
}


// Example of an "internal" module
mod built_in_plugins{
    use linkme::distributed_slice;
    use crate::{Plugin, PLUGINS};

    #[distributed_slice(PLUGINS)]
    static AGE_PLUGIN: Plugin = Plugin{
        name: "Age",
        description: "Age of the person",
        required: true,
        enabled: true,
        initializer: |_context, _person_id| {
            // The default age.
            42;
        },
        constructor: |context| {
            context.register_plugin(&AGE_PLUGIN);
        }
    };
}


#[cfg(test)]
mod test {
    use super::Context;

    #[test]
    fn it_works() {
        let context = Context::new();
        for plugin in context.plugins.iter() {
            println!("Plugin: {}", plugin);
        }
    }
}