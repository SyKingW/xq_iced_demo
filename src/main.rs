/// UI 库
use iced::{
    button, executor, pane_grid, pick_list, scrollable, slider, text_input, tooltip, Align,
    Application, Button, Checkbox, Color, Column, Command, Container, Element, Font,
    HorizontalAlignment, Length, PaneGrid, PickList, ProgressBar, Radio, Row, Rule, Scrollable,
    Settings, Slider, Text, TextInput, Tooltip, VerticalAlignment,
};

use std::ops::RangeInclusive;

use std::borrow::Cow;

// 用 include_bytes 如果路径错误，还会提示的
const XQFONT: Font = Font::External {
    name: "方正字体",
    bytes: include_bytes!("source/FZKT_Document.TTF"),
};

fn main() -> iced::Result {
    println!("Hello, world!");
    Hello::run(Settings::default())
}

/// UI struct
struct Hello {
    text_str: String,

    button_state: button::State,

    input_str: String,
    input_state: text_input::State,

    slider_value: i32,
    slider_state: slider::State,

    is_checked: bool,

    pane_grid_state: pane_grid::State<PaneState>,

    pick_list_str: String,
    pick_list_state: pick_list::State<String>,

    radio_value: bool,

    scrollable_state: scrollable::State,
}

/// Hello 的 message 类型
#[derive(Debug, Clone)]
enum HelloMessage {
    Button,

    TextInput(String),
    TextInputSubmit,

    Slider(i32),
    SliderRelease,

    CheckBox(bool),

    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    PaneClick(pane_grid::Pane),

    PickList(String),

    Radio(bool),
}

/// 这个用 i32 也行呢
enum PaneState {
    Horizontal,
    Vertical,
}

/// 遵守 iced 的特性
/// 第一次 new -> title -> title -> view
/// 后面更新视图则是  update -> title -> view
impl Application for Hello {
    type Executor = executor::Default;
    type Message = HelloMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        println!("new");

        (
            Hello {
                text_str: String::from("我是 Text"),
                button_state: button::State::new(),

                input_str: String::from(""),
                input_state: text_input::State::new(),

                slider_value: 30,
                slider_state: slider::State::new(),

                is_checked: false,

                pane_grid_state: pane_grid::State::new(PaneState::Vertical).0,

                pick_list_str: String::from(""),
                pick_list_state: pick_list::State::default(),

                radio_value: false,

                scrollable_state: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        println!("title");
        // 标题就算没有字体，也能显示中文，但是 iced 的控件不行
        // String::from("Hello world")
        String::from("你好，世界")
    }

    fn view(&mut self) -> Element<Self::Message> {
        println!("view");

        // text
        let text = Text::new(&self.text_str)
        .size(20) // 字体大小
        .height(Length::Units(100)) // 高度(默认自适应)
        .width(Length::Units(200)) // 宽度(默认自适应)
        .color(Color::new(0.2, 0.8, 0.2, 1.0)) // 字颜色
        .font(XQFONT) // 字体
        .vertical_alignment(VerticalAlignment::Center) // 字纵向的排列
        .horizontal_alignment(HorizontalAlignment::Center) // 字横向的排列
        ;

        // textinput
        let text_input = TextInput::new(
            &mut self.input_state,
            "TextInput",
            &self.input_str,
            HelloMessage::TextInput,
        )
        .on_submit(HelloMessage::TextInputSubmit) // 用户按下回车, 发送消息
        .padding(10) // 内间距
        .width(Length::Units(200))
        .password() // 变成密码类型输入框
        ;

        // button
        let xq_button = Button::new(&mut self.button_state, Text::new("Touch me!(Button)"))
            .on_press(HelloMessage::Button) // 点击发出 message
            .min_width(250) // 最小宽度
            .min_height(60) // 最小高度
            .padding(20) // 内间距
            ;

        // slider
        let mut slider_value_str = self.slider_value.to_string();
        slider_value_str.push_str(" Slider Value");
        let slider_text = Text::new(slider_value_str);
        let xq_slider = Slider::new(
            &mut self.slider_state,
            RangeInclusive::new(0, 100),
            self.slider_value,
            HelloMessage::Slider,
        )
        .width(Length::Units(300))
        .step(2) // 一次跳转多少
        .on_release(HelloMessage::SliderRelease) // 放开 Slider 通知
        ;

        // CheckBox
        let check_box = Checkbox::new(self.is_checked, "CheckBox", HelloMessage::CheckBox)
        .size(30) // 框大小
        .spacing(10) // 框和字的间距
        .text_size(30) // 字大小
        ;

        // PaneGrid
        let xq_pane_grid = PaneGrid::new(&mut self.pane_grid_state, |_, state| {
            pane_grid::Content::new(match state {
                PaneState::Vertical => Text::new("我是 pane0, 点我纵向切割").font(XQFONT),
                PaneState::Horizontal => Text::new("我是 pane1, 点我横向切割").font(XQFONT),
            })
        })
        .on_drag(HelloMessage::PaneDragged) // 拖动事件
        .on_resize(10, HelloMessage::PaneResized) // 刷新布局事件
        .on_click(HelloMessage::PaneClick) // 点击事件
        .height(Length::Units(500))
        .spacing(10) // 控件间距
        ;

        // Container
        let xq_container = Container::new(Text::new("Container"));

        //  PickList
        let pick_list_option = vec![String::from("111"), String::from("222")];
        let cow = Cow::Owned(pick_list_option);
        let xq_pick_list = PickList::new(
            &mut self.pick_list_state,
            cow,
            Option::Some(self.pick_list_str.clone()),
            HelloMessage::PickList,
        )
        .padding(10) // 内间距
        .text_size(30) // 文字大小
        ;

        // ProgressBar
        let xq_progress_bar =
            ProgressBar::new(RangeInclusive::new(0.0, 100.0), self.slider_value as f32);

        // Radio
        let xq_radio = Radio::new(
            true,
            "Radio".to_string(),
            Option::Some(self.radio_value),
            HelloMessage::Radio,
        )
        .size(30) // 框大小
        .text_size(30) // 字大小
        .spacing(10) // 框和字的间距
        ;

        // Rule
        let xq_rule = Rule::horizontal(30);

        // Tooltip
        let xq_tooltip = Tooltip::new(
            Text::new("移动到我这里，会出现 Tooltip").font(XQFONT),
            "我是 Tooltip",
            tooltip::Position::Top,
        )
        .gap(10) // 和 content 的距离
        .font(XQFONT)
        .padding(10) // 提示内间距
        .size(30) // 提示字大小
        ;

        let column = Column::new()
            .push(text)
            .push(text_input)
            .push(xq_button)
            .push(
                Row::new()
                    .push(Text::new("Row layout 0").color(Color::new(0.6, 0.5, 0.9, 1.0)))
                    .push(Text::new("Row layout 1"))
                    .push(Text::new("Row layout 2"))
                    .spacing(10),
            )
            .push(slider_text)
            .push(
                Row::new()
                    .push(Text::new("0"))
                    .push(xq_slider)
                    .push(Text::new("100")),
            )
            .push(xq_progress_bar)
            .push(check_box)
            .push(xq_container)
            .push(xq_pick_list)
            .push(xq_radio)
            .push(xq_rule)
            .push(xq_tooltip)
            .push(xq_pane_grid)
            .padding(16) // 内间距
            .spacing(12) // 每个控件的间距
            .align_items(Align::Start) // 布局对齐方式，默认就是start
            .max_height(1000000) // 最大高度
            .max_width(1000000) // 最大宽度
            ; 

        // Scrollable
        Scrollable::new(&mut self.scrollable_state)
            .push(column) // 添加控件
            .into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _: &mut iced::Clipboard,
    ) -> Command<Self::Message> {
        println!("update");
        match message {
            HelloMessage::Button => {
                self.text_str = String::from("Touch Button(Text)");
            }

            HelloMessage::TextInput(value) => {
                self.input_str = value;
            }
            HelloMessage::TextInputSubmit => {
                println!("TextInputSubmit 按回车");
            }

            HelloMessage::Slider(value) => {
                self.slider_value = value;
            }
            HelloMessage::SliderRelease => {
                println!("SliderRelease 放开Slider");
            }

            HelloMessage::CheckBox(value) => {
                self.is_checked = value;
            }

            HelloMessage::PaneDragged(value) => {
                println!("PaneDragged: {:?}", value);
            }
            HelloMessage::PaneResized(value) => {
                println!("PaneResized: {:?}", value);
            }

            HelloMessage::PaneClick(pane) => {
                println!("PaneClick: {:?}", pane);

                match self.pane_grid_state.get(&pane) {
                    Some(value) => {
                        match value {
                            PaneState::Vertical => {
                                // 分离模块
                                self.pane_grid_state.split(
                                    pane_grid::Axis::Vertical,
                                    &pane,
                                    PaneState::Horizontal,
                                );
                            }
                            PaneState::Horizontal => {
                                self.pane_grid_state.split(
                                    pane_grid::Axis::Horizontal,
                                    &pane,
                                    PaneState::Vertical,
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
            HelloMessage::PickList(value) => {
                println!("PickList: {}", value);
                self.pick_list_str = value;
            }
            HelloMessage::Radio(value) => {
                println!("Radio: {}", value);
                self.radio_value = !self.radio_value;
            }
            
        }

        Command::none()
    }
}
