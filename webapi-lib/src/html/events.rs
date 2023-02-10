use paste::paste;

trait HtmlEvent {
    fn call(&self, f: Box<dyn Fn()>);
}

trait EventData {
    fn data(&self) -> Box<dyn EventData> {
        Box::new(ClickEv {})
    }
}

struct Event<E> 
where E: EventData {
    data: Box<E>,
}
impl<E: EventData> Event<E> {

}


struct ClickEv {}
impl EventData for ClickEv {}

impl Event<ClickEv> {
    fn get(&self) -> &ClickEv {
        self.data.as_ref()
    }
}

macro_rules! create_event {
    (
        $( $name:ident ),* $(,)*
    ) => {paste! {
        $(
            pub struct [<On $name>] {}
            impl HtmlEvent for [<On $name>] {
                fn call(&self, f: Box<dyn Fn()>) {
                    f();
                }
            }
        )*

    }}
}

create_event!(
    Abort,
    AfterPrint,
    BeforePrint,
    BeforeUnload,
    Blur,
    Cancel,
    CanPlay,
    CanPlayThrough,
    Change,
    Click,
    Close,
    ContextMenu,
    CueChange,
    DblClick,
    Drag,
    DragEnd,
    DragEnter,
    DragLeave,
    DragOver,
    DragStart,
    Drop,
    DurationChange,
    Emptied,
    Ended,
    Error,
    Focus,
    FullScreenChange,
    FullScreenError,
    HashChange,
    Input,
    Invalid,
    KeyDown,
    KeyPress,
    KeyUp,
    Load,
    LoadedData,
    LoadedMetadata,
    LoadStart,
    Message,
    MouseDown,
    MouseEnter,
    MouseLeave,
    MouseMove,
    MouseOut,
    Mouseover,
    MouseUp,
    Offline,
    Online,
    Open,
    PageHide,
    PageShow,
    Pause,
    Play,
    Playing,
    PopState,
    Progress,
    RateChange,
    Reset,
    Resize,
    Scroll,
    Search,
    SeekEd,
    Seeking,
    Select,
    Stalled,
    Storage,
    Submit,
    Suspend,
    TimeUpdate,
    Toggle,
    TouchCancel,
    TouchEnd,
    TouchMove,
    TouchStart,
    TransitionEnd,
    Unload,
    VolumeChange,
    Waiting,
    Wheel,
);

impl OnAbort {
    
}

// impl HtmlEvent for OnAbort {
//     fn call(&self, f: Box<dyn Fn()>) {
//         f();
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_event() {
        let ev = OnClick {};

        ev.call(Box::new(|| {}));

        let click = ClickEv {};
        click.data();

    }
}
