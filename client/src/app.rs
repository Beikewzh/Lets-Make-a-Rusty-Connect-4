use crate::{
	components::{
		connect4_page::Connect4Page, auth::AuthPage, sidebar::SideBar, stats::Stats,
		toot_and_otto_page::TootAndOttoPage,
	},
	switch::{AppRoute, AppRouter, PublicUrlSwitch},
};
use yew::prelude::*;

pub struct App {}

pub enum Message {}

impl Component for App {
	type Message = Message;
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
		let get_route = AppRouter::render(|switch: PublicUrlSwitch| match switch.route() {
			AppRoute::Connect4 => html! {<Connect4Page />},
			AppRoute::Login => html! {<AuthPage/>},
			AppRoute::Stats => html! {<Stats />},
			AppRoute::TootAndOtto => html! {<TootAndOttoPage />},
			AppRoute::Home => html! {<AuthPage/>},
		});

		html! {
			<div class="app">
				<SideBar />
				<AppRouter render=get_route />
			</div>
		}
	}
}
