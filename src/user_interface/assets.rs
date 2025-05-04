use iced::widget::image;

#[derive(Debug)]
pub struct MinesweeperAssets {
    // #region Field tiles
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
    pub mine_false: image::Handle,
    pub mine_detonated: image::Handle,
    pub question_closed: image::Handle,
    pub question_open: image::Handle,

    //#endregion

    //#region Score indicators
    pub score0: image::Handle,
    pub score1: image::Handle,
    pub score2: image::Handle,
    pub score3: image::Handle,
    pub score4: image::Handle,
    pub score5: image::Handle,
    pub score6: image::Handle,
    pub score7: image::Handle,
    pub score8: image::Handle,
    pub score9: image::Handle,
    pub score_empty: image::Handle,
    pub score_dash: image::Handle,

    //#endregion

    //#region Face
    pub face: image::Handle,
    pub face_pressed: image::Handle,
    pub face_lose: image::Handle,
    pub face_win: image::Handle,
    pub face_open: image::Handle, //#endregion
}
impl Default for MinesweeperAssets {
    fn default() -> Self {
        Self {
            // #region Field tiles
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
            mine_false: image::Handle::from_bytes(
                include_bytes!("../../resources/images/field/mine_false.png").as_slice(),
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

            //#endregion

            //#region Score indicators
            score0: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/0.png").as_slice(),
            ),
            score1: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/1.png").as_slice(),
            ),
            score2: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/2.png").as_slice(),
            ),
            score3: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/3.png").as_slice(),
            ),
            score4: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/4.png").as_slice(),
            ),
            score5: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/5.png").as_slice(),
            ),
            score6: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/6.png").as_slice(),
            ),
            score7: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/7.png").as_slice(),
            ),
            score8: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/8.png").as_slice(),
            ),
            score9: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/9.png").as_slice(),
            ),
            score_dash: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/dash.png").as_slice(),
            ),
            score_empty: image::Handle::from_bytes(
                include_bytes!("../../resources/images/score/empty.png").as_slice(),
            ),
            //#endregion

            //#region Face
            face: image::Handle::from_bytes(
                include_bytes!("../../resources/images/face/face.png").as_slice(),
            ),
            face_pressed: image::Handle::from_bytes(
                include_bytes!("../../resources/images/face/face_pressed.png").as_slice(),
            ),
            face_lose: image::Handle::from_bytes(
                include_bytes!("../../resources/images/face/lose.png").as_slice(),
            ),
            face_win: image::Handle::from_bytes(
                include_bytes!("../../resources/images/face/win.png").as_slice(),
            ),
            face_open: image::Handle::from_bytes(
                include_bytes!("../../resources/images/face/open.png").as_slice(),
            ),
            //#endregion
        }
    }
}
