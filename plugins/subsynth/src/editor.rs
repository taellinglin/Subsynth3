use nih_plug::prelude::{Editor};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

use std::sync::Arc;

use crate::SubSynthParams;

// zCool font constant
const ZCOOL_XIAOWEI: &str = "ZCOOL XiaoWei";
const ZCOOL_FONT_DATA: &[u8] = include_bytes!("assets/ZCOOL_XIAOWEI_REGULAR.ttf");

#[derive(Lens)]
struct Data {
    params: Arc<SubSynthParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (840, 480))
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
        // Register zCool font
        cx.add_fonts_mem(&[ZCOOL_FONT_DATA]);
        
        // Set zCool as the default font for the entire UI
        cx.set_default_font(&[ZCOOL_XIAOWEI]);

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);
        Label::new(cx, "SubSynth")
            .font_family(vec![FamilyOwned::Name(String::from(ZCOOL_XIAOWEI))])
            .font_size(32.0)
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
                Label::new(cx, "Env Int")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.amp_envelope_level);
            });

            VStack::new(cx, |cx| {
                Label::new(cx, "Filter Cut Atk")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_cut_attack_ms);
                Label::new(cx, "Filter Cut Dec")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_cut_decay_ms);
                Label::new(cx, "Filter Cut Sus")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_cut_sustain_ms);
                Label::new(cx, "Filter Cut Rel")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_cut_release_ms);
                Label::new(cx, "Amount")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_cut_envelope_level);
            });
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

                Label::new(cx, "Filter Q Rel")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_res_release_ms);
                Label::new(cx, "Amount")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_res_envelope_level);
            })
            .row_between(Pixels(0.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));

        });
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
            
                Label::new(cx, "Vib Int")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.vibrato_intensity);

                Label::new(cx, "Vib Rate")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.vibrato_rate);
            });
            VStack::new(cx, |cx| {
                
                Label::new(cx, "Vib Attack")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.vibrato_attack);
                
                Label::new(cx, "Vib Shape")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.vibrato_shape);
            });
            VStack::new(cx, |cx| {
            
                Label::new(cx, "Trem Int")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.tremolo_intensity);

                Label::new(cx, "Tremo Rate")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.tremolo_rate);
            });
            VStack::new(cx, |cx| {
                
                Label::new(cx, "Tremo Atk")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.tremolo_attack);

                
                Label::new(cx, "Tremo Shape")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                ParamSlider::new(cx, Data::params.clone(), |params| &params.tremolo_shape);

            })
            .row_between(Pixels(0.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));
            
        });

    })
}
                
