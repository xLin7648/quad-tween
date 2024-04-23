# This is an easing library available for macroquad


## Quick start

Add quad-tween as a dependency to Cargo.toml:
```toml

[dependencies]
quad_tween = { git = "https://github.com/xLin7648/quad-tween.git" }
```

#### First introduce the library

```rust
	use quad_tween::*
```

#### Then create an animation
```rust
let mut points = Vec::new();

points.push(vec2(400. , 600.));
points.push(vec2(800. , 600.));
points.push(vec2(1200., 200.));
points.push(vec2(800. , 200.));

let tween: Tween<Vec2> = Tween::<Vec2>::new(
	Ease::InOutExpo,
	true, // isloop?
	1.5,  // total time,
	points
);
```

#### Then update it every frame
```rust
loop {
	clear_background(BLUE);

	tween.update(|tw| {
		draw_circle(tw.points[0].x , tw.points[0].y , 20., BLACK);
		draw_circle(tw.points[1].x , tw.points[1].y , 20., WHITE);
		draw_circle(tw.points[2].x , tw.points[2].y , 20., BLACK);
		draw_circle(tw.points[3].x , tw.points[3].y , 20., WHITE);
		draw_circle(tw.result.x    , tw.result.y    , 20., GREEN);
    });

	next_frame().await
}

```