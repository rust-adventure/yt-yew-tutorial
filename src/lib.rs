use gloo_net::http::Request;
use yew::prelude::*;

mod video;

use video::*;

pub enum Msg {
    SetVideos(Vec<Video>),
    Select(Video),
    Error,
}

pub struct App {
    videos: Vec<Video>,
    selected: Option<Video>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let fetched_videos: Vec<Video> =
                Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            Msg::SetVideos(fetched_videos)
        });
        Self {
            videos: vec![],
            selected: None,
        }
    }

    fn update(
        &mut self,
        _ctx: &Context<Self>,
        msg: Self::Message,
    ) -> bool {
        match msg {
            Msg::SetVideos(videos) => {
                self.videos = videos;
                true
            }
            Msg::Error => todo!(),
            Msg::Select(video) => {
                self.selected = Some(video.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let details = self.selected.as_ref().map(|video| {
            html! {
                <VideoDetails video={video.clone()} />
            }
        });

        html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList
                  videos={self.videos.clone()}
                  on_click={ctx.link().callback(|video: Video| Msg::Select(video))}
                />
            </div>
            { for details }
        </>
        }
    }
}
