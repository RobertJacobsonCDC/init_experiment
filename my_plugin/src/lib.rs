use linkme::distributed_slice;
use initialization::{Plugin, PLUGINS};

#[distributed_slice(PLUGINS)]
static WEIGHT_PLUGIN: Plugin = Plugin{
  name: "Weight",
  description: "Weight of the person in lbs",
  required: true,
  enabled: true,
  initializer: |_context, _person_id| {
    // The default weight.
    140;
  },
  constructor: |context| {
    context.register_plugin(&WEIGHT_PLUGIN);
  }
};


#[cfg(test)]
mod test {
  use initialization::Context;

  #[test]
   fn it_works() {
    let context = Context::new();
    for plugin in context.plugins.iter() {
      println!("Plugin: {}", plugin);
    }
   } 
}