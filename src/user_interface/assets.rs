use iced::widget::image;

#[derive(Debug)]
pub struct MinesweeperFieldAssets {
    pub field0: image::Handle,
    pub field1: image::Handle,
    pub field2: image::Handle,
    pub field3: image::Handle,
    pub field4: image::Handle,
    pub field5: image::Handle,
    pub field6: image::Handle,
    pub field7: image::Handle,
    pub field8: image::Handle,
    pub closed: image::Handle,
    pub flag: image::Handle,
    pub mine: image::Handle,
    pub mine_defused: image::Handle,
    pub mine_detonated: image::Handle,
    pub question_closed: image::Handle,
    pub question_open: image::Handle,
}
impl Default for MinesweeperFieldAssets {
    fn default() -> Self {
        Self {
            field0: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/0.png").as_slice(),
            ),
            field1: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/1.png").as_slice(),
            ),
            field2: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/2.png").as_slice(),
            ),
            field3: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/3.png").as_slice(),
            ),
            field4: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/4.png").as_slice(),
            ),
            field5: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/5.png").as_slice(),
            ),
            field6: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/6.png").as_slice(),
            ),
            field7: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/7.png").as_slice(),
            ),
            field8: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/8.png").as_slice(),
            ),
            closed: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/closed.png").as_slice(),
            ),
            flag: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/flag.png").as_slice(),
            ),
            mine_defused: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/mine_defused.png").as_slice(),
            ),
            mine_detonated: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/mine_detonated.png").as_slice(),
            ),
            mine: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/mine.png").as_slice(),
            ),
            question_closed: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/question_closed.png").as_slice(),
            ),
            question_open: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/question_open.png").as_slice(),
            ),
        }
    }
}
