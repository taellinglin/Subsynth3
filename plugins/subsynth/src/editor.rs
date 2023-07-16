use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

use std::sync::Arc;

use crate::SubSynthParams;

#[derive(Lens)]
struct Data {
    params: Arc<SubSynthParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (860, 420))
}

fn create_label<'a, T>(
    cx: &'a mut Context,
    text: impl Res<T>,
    height: f32,
    width: f32,
    child_top: f32,
    child_bottom: f32,
) where
    T: ToString,
{
    Label::new(cx, text)
        .height(Pixels(height))
        .width(Pixels(width))
        .child_top(Stretch(child_top))
        .child_bottom(Pixels(child_bottom));
}

pub(crate) fn create(
    params: Arc<SubSynthParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);
        Label::new(cx, "SubSynth")
            .font_family(vec![FamilyOwned::Name(String::from(
                assets::NOTO_SANS_LIGHT,
            ))])
            .font_size(32.0) // increase the font size to 24
            .height(Pixels(50.0))
            .width(Stretch(1.0))
            .child_top(Stretch(1.0))
            .child_bottom(Pixels(0.0));
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Gain")
                    .height(Pixels(20.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));

                ParamSlider::new(cx, Data::params.clone(), |params| &params.gain);
                create_label(cx, "Waveform", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| &params.waveform);
                create_label(cx, "Filter Type", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_type);
                create_label(cx, "Filter Cut", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_cut);
                create_label(cx, "Filter Res", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_res);
            });

            VStack::new(cx, |cx| {
                create_label(cx, "Attack", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| &params.amp_attack_ms);
                create_label(cx, "Decay", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| &params.amp_decay_ms);
                create_label(cx, "Sustain", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| &params.amp_sustain_level);
                create_label(cx, "Release", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| &params.amp_release_ms);
            });

            VStack::new(cx, |cx| {
                create_label(cx, "Filter Cut Atk", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| {
                    &params.filter_cut_attack_ms
                });
                create_label(cx, "Filter Cut Dec", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| {
                    &params.filter_cut_decay_ms
                });
                create_label(cx, "Filter Cut Sus", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| {
                    &params.filter_cut_sustain_ms
                });
                create_label(cx, "Filter Cut Rel", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| {
                    &params.filter_cut_release_ms
                });
            })
            .row_between(Pixels(0.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));
            VStack::new(cx, |cx| {
                create_label(cx, "Filter Q Atk", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| {
                    &params.filter_res_attack_ms
                });
                create_label(cx, "Filter Q Dec", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| {
                    &params.filter_res_decay_ms
                });
                create_label(cx, "Filter Q Sus", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| {
                    &params.filter_res_sustain_ms
                });

                create_label(cx, "Filter Q Rel", 20.0, 100.0, 1.0, 0.0);
                ParamSlider::new(cx, Data::params.clone(), |params| {
                    &params.filter_res_release_ms
                });
            })
            .row_between(Pixels(0.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));
        });
    })
}
