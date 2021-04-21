#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
	Left = 1 << 0,
	Right = 1 << 1,
	Middle = 1 << 2,
	Move = 1 << 3,
	WheelUp = 1 << 4,
	WheelDown = 1 << 5,
	Wheel = 1 << 6,
	None = 1 << 7,
	Unknown = 1 << 8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
	Resize(u32, u32),
	PixelResize(u32, u32),
	FontResize(u32, u32),
	MouseMove(MouseButton, u32, u32),
	MouseUp(MouseButton, u32, u32),
	MouseDown(MouseButton, u32, u32),
	KeyPress(char),
	Unknown,
}
