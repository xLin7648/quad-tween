use std::f32::consts::PI;

pub enum Ease {
    InSine,
    OutSine,
    InOutSine,
    InQuad,
    OutQuad,
    InOutQuad,
    InCubic,
    OutCubic,
    InOutCubic,
    InQuart,
    OutQuart,
    InOutQuart,
    InQuint,
    OutQuint,
    InOutQuint,
    InExpo,
    OutExpo,
    InOutExpo,
    InCirc,
    OutCirc,
    InOutCirc,
    InBack,
    OutBack,
    InOutBack,
    InElastic,
    OutElastic,
    InOutElastic,
    InBounce,
    OutBounce,
    InOutBounce
}

pub fn easeing(ease: &Ease, x: f32) -> f32 {
    match ease {
        Ease::InSine     => 1. - f32::cos((x * PI) * 0.5),
        Ease::OutSine    => f32::sin((x * PI) * 0.5),
        Ease::InOutSine  => -(f32::cos(PI * x) - 1.) * 0.5,
        Ease::InQuad     => x * x,
        Ease::OutQuad    => 1. - (1. - x) * (1. - x),
        Ease::InOutQuad  => if x < 0.5 { 2. * x * x } else { 1. - f32::powi(-2. * x + 2., 2) * 0.5 },
        Ease::InCubic    => x * x * x,
        Ease::OutCubic   => 1. - f32::powi(1. - x, 3),
        Ease::InOutCubic => if x < 0.5 { 4. * x * x * x } else { 1. - f32::powi(-2. * x + 2., 3) * 0.5 },
        Ease::InQuart    => x * x * x * x,
        Ease::OutQuart   => 1. - f32::powi(1. - x, 4),
        Ease::InOutQuart => if x < 0.5 { 8. * x * x * x * x } else { 1. - f32::powi(-2. * x + 2., 4) * 0.5 },
        Ease::InQuint    => x * x * x * x * x,
        Ease::OutQuint   => 1. - f32::powi(1. - x, 5),
        Ease::InOutQuint => if x < 0.5 { 16. * x * x * x * x * x } else { 1. - f32::powi(-2. * x + 2., 5) * 0.5 },
        Ease::InExpo     => if x != 0. { f32::powf(2., 10. * x - 10.) } else { 0. },
        Ease::OutExpo    => if x != 1. { 1. -f32::powf(2., -10. * x)  } else { 1. },
        Ease::InOutExpo  => {
            if x != 0. {
                if x != 1. {
                    if x < 0.5 {
                        f32::powf(2., 20. * x - 10.) * 0.5
                      } else {
                        (2. - f32::powf(2., -20. * x + 10.)) * 0.5
                      }
                } else { 1. }
            } else { 0. }
        },
        Ease::InCirc    => 1. - f32::sqrt(1. - f32::powi(x, 2)),
        Ease::OutCirc   => f32::sqrt(1. - f32::powi(x - 1., 2)),
        Ease::InOutCirc => {
            if x < 0.5 {
                (1. - f32::sqrt(1. - f32::powi(2. * x, 2))) * 0.5
              } else {
                (f32::sqrt(1. - f32::powi(-2. * x + 2., 2)) + 1.) * 0.5
              }
        },
        Ease::InBack => {
            let c1 = 1.70158;
            let c3 = c1 + 1.;
            c3 * x * x * x - c1 * x * x
        },
        Ease::OutBack => {
            let c1 = 1.70158;
            let c3 = c1 + 1.;

            1. + c3 * f32::powi(x - 1., 3) + c1 * f32::powi(x - 1., 2)
        },
        Ease::InOutBack => {
            let c1 = 1.70158;
            let c2 = c1 * 1.525;

            if x < 0.5 {
                (f32::powi(2. * x, 2) * ((c2 + 1.) * 2. * x - c2)) * 0.5
            } else {
                (f32::powi(2. * x - 2., 2) * ((c2 + 1.) * (x * 2. - 2.) + c2) + 2.) * 0.5
            }
        },
        Ease::InElastic => {
            let c4 = (2. * PI) / 3.;

            if x != 0. {
                if x != 1. {
                    -f32::powf(2., 10. * x - 10.) * f32::sin((x * 10. - 10.75) * c4)
                } else { 1. }
            } else { 0. }
        },
        Ease::OutElastic => {
            let c4 = (2. * PI) / 3.;

            if x != 0. {
                if x != 1. {
                    f32::powf(2., -10. * x) * f32::sin((x * 10. - 0.75) * c4) + 1.
                } else { 1. }
            } else { 0. }
        },
        Ease::InOutElastic => {
            let c5 = (2. * PI) / 4.5;

            if x == 0. {
                if x == 1. {
                    if x < 0.5 {
                        -(f32::powf(2., 20. * x - 10.) * f32::sin((20. * x - 11.125) * c5)) * 0.5
                    } else {
                        (f32::powf(2., -20. * x - 10.) * f32::sin((20. * x - 11.125) * c5)) * 0.5 + 1.
                    }
                  } else { 1. }
              } else { 0. }
        },
        Ease::InBounce => 1. - easeing(&Ease::OutBounce, 1. - x),
        Ease::OutBounce => {
            let n1 = 7.5625;
            let d1 = 2.75;

            if x < 1. / d1 {
                n1 * x * x
            } else if x < 2. / d1 {
                let x = x - 1.5;
                n1 * (x / d1) * x + 0.75
            } else if x < 2.5 / d1 {
                let x = x - 2.25;
                n1 * (x / d1) * x + 0.9375
            } else {
                let x = x - 2.625;
                n1 * (x / d1) * x + 0.984375
            }
        },
        Ease::InOutBounce => {
            if x < 0.5 {
                (1. - easeing(&Ease::OutBounce, 1. - 2. * x)) * 0.5
            } else {
                (1. + easeing(&Ease::OutBounce, 2. * x - 1.)) * 0.5
            }
        }
    }
}
