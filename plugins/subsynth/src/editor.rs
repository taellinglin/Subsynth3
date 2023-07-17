use nih_plug::prelude::{Editor};
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
                Label::new(cx, "Waveform")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.waveform);
                Label::new(cx, "Filter Type")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_type);
                Label::new(cx, "Filter Cut")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_cut);
                Label::new(cx, "Filter Res")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_res);
                
            });
            

            VStack::new(cx, |cx| {
                Label::new(cx, "Attack")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.amp_attack_ms);
                Label::new(cx, "Decay")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.amp_decay_ms);
                Label::new(cx, "Sustain")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.amp_sustain_level);
                Label::new(cx, "Release")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.amp_release_ms);
                Label::new(cx, "Envelope Level")
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
                Label::new(cx, "Filter Q Atk")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_res_attack_ms);
                Label::new(cx, "Filter Q Dec")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_res_decay_ms);
                Label::new(cx, "Filter Q Sus")
                    .height(Pixels(20.0))
                    .width(Pixels(100.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));
                    
                ParamSlider::new(cx, Data::params.clone(), |params| &params.filter_res_sustain_ms);

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
    }
)}
                
