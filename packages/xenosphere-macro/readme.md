# xenosphere-macro

## Abstract 

to convert the macro input to the token-steam

```rs


// create the modifier

// src/themes/component_name.rs
create_modifier_map! {
    component_modifier_name {
      padding: 10,
      margin: 10,
      color: "red",
      background: "blue",
    }
}


#[component]
fn main_component (
  #[props] props_string   : State<String>,
  #[props] props_int      : State<Int>,
  #[event] event_click    : EventHandler<Click>,

) -> impl ComponentView {

  let local_state = state!(0);
  let dync_state =  computed!( move || local_state.get() + 1);

  let custom_event_dispatcher = event_dispatcher!("event_a", move |event: Click| {
    println!("click event");
  });

  on_created!(move|current_self|{
    println!("component created");
  });

  creat_effect!({
    println!("component updated");
  });

  watch!(local_state, |curr, prev| async move{
    println!("component updated");

    custom_event_dispatcher.dispatch!(curr, prev);
  });

  on_destroyed!({
    println!("component destroyed");
  });

  !view {
    Box {
      component_ref!(package_name::component_name, Mode::Overwrite),

      modifier: create_modifier! {
        // inherit the predefine modifier
        modifier_ref!(package_name::component_modifier_name),
        // define the modifier
        padding: 10,
        margin: 10,
        color: "red",
        background: "blue",
      },

      event_handler: create_event_modifiers!{
        // inherit the predefine event handler
        event_handler_ref!(package_name::event_handler_name),
        // define the event handler
        click: [
          event_handler!(event_click),
        ],
        // or define the event handler
      },


      // define the child component
      {
        Text {
          modifier: {
            padding: 10,
            margin: 10,
            color: "red",
            background: "blue",
          },
          (props_string),
        },
        Spacer {
          size: 10,
        },
      }
    }
  }

}

```

then convert into final component code

```rs

pub struct MainComponentProps {
  props_string: State<String>,
  props_int: State<Int>,
  event_click: EventHandler<Click>,
}

struct MainComponent {
  props: MainComponentProps,
  local_state: StateToken<Int>,
  dync_state: ComputedToken<Int>,
  custom_event_dispatcher: EventDispatcher,
}

enum ModifierVarient {
  Int(i32),
  String(String),
  DynamicState(String),
}

struct RawModifier = HashMap<String, Varient>;

struct ModifierTokenSet {
  _component_name: String,
  _component_modifier_name: String, 
  _raw_modifier: RawModifier,
}


/// ModifierTokenSet : based on the CodeBuilder to generate the code
impl ModifierTokenSet {
  pub fn to_code(&self) -> String {
    let mut code = Vec::<String>::new();
    for (key, value) in self._raw_modifier {
      // code.push_str(&format!("{}: {},", key, value));
      
      // e.g. html-style 
      match value {
        ModifierVarient::Int(value) => {
          code.push(&format!("{}: {}px;", key, value));
        },
        ModifierVarient::String(value) => {
          code.push(&format!("{}: {};", key, value));
        },
        ModifierVarient::DynamicState(value) => {
          code.push(&format!("{}: --{}-{};", key, key));
        },
      }
      code.push(&format!("{}: {};", key, value));
    }
    code.join("\n")
  }
}

struct ViewToken {
  component_ref: ComponentRefToken,
  modifier: ModifierTokenSet,
  event_handler: EventHandlerToken,
  children: Vec<Box<dyn ComponentView>>,
}


/// then pass to code generator
/// 
/// 
/// 

impl ViewToken {
  pub fn to_code(&self) -> String {
    let mut code = String::new();
    code.push_str(&self.component_ref.to_code());
    code.push_str(&self.modifier.to_code());
    code.push_str(&self.event_handler.to_code());
    code.push_str(&self.children.to_code());
    code
  }
}

pub fn main_process(tokens: Vec<ViewToken>) -> Result<String, Error> {
  for token in tokens {
    let component = MainComponent {
      props: MainComponentProps {
        props_string: token.props_string,
        props_int: token.props_int,
        event_click: token.event_click,
      },
      local_state: token.local_state,
      dync_state: token.dync_state,
      custom_event_dispatcher: token.custom_event_dispatcher,
    };

    let view = component.view();
    let code = view.to_code();
    println!("{}", code);
  }
}

```