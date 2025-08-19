use dioxus::prelude::*;
pub mod core;
use crate::myw;
#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            style: "text-align: center",
            myw::Gap {  }
            core::Logo {  }
            myw::Gap {  }
            core::Search {  }
            myw::Gap {  }
            myw::Gap {  }
            core::Nav {  }
        }

    }
}
