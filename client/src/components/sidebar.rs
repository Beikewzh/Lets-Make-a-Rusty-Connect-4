use crate::switch::{AppRoute, RouterButton};
use yew::prelude::*;

pub struct SideBar {}

pub enum Msg {}

impl Component for SideBar {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self {}
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div class="sidebar">
				<div class="sidebar__urls">
					<RouterButton route=AppRoute::Connect4> {"CONNECT4"} </RouterButton>
					<RouterButton route=AppRoute::TootAndOtto> {"TOOT&OTTO"} </RouterButton>
					<RouterButton route=AppRoute::Stats> {"STATS"} </RouterButton>
					<RouterButton route=AppRoute::Login> {"AUTH"} </RouterButton>
				</div>
			</div>
		}
	}
}
