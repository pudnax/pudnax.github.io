package main

import (
	"math"
	"math/rand"

	p "github.com/bregydoc/PGoJs/Processing"
)

var widht = 1535.
var height = 1241.
var current = newParticle(0, 0)
var flow = []*Particle{current}
var r = 5.

func setup() {
	p.CreateCanvas(int(widht), int(height))
	p.Background(0)
	// p.FullScreen()
}

func draw() {
	p.Translate(widht/2, height/2)
	p.Background(0)
	p.Stroke(255)
	p.Rotate(math.Pi / 6)

	for !current.finished() && !current.intersects(flow) {
		current.update()
	}
	current.show()

	// for current.finished() || current.intersects(flow) {
	// 	current.update()
	// 	current.show()
	// }

	flow = append(flow, current)
	current = newParticle(widht/2, 0)

	for i := 0; i < 6; i++ {
		p.Rotate(math.Pi / 3)

		current.show()
		for _, val := range flow {
			val.show()
		}

		p.Push()
		p.Scale(1, -1)
		current.show()
		for _, val := range flow {
			val.show()
		}
		p.Pop()
	}
}

type Particle struct {
	X, Y float64
	R    float64
}

func (P *Particle) update() {
	P.X -= 1
	P.Y += rand.Float64()*8 - 4

}

func (P *Particle) finished() bool {
	return P.X < 0
}

func newParticle(x, y float64) *Particle {
	return &Particle{X: x, Y: y, R: r}
}

func (P *Particle) show() {
	p.Ellipse(P.X, P.Y, P.R, P.R)
}

func (P *Particle) intersects(arr []*Particle) bool {
	res := false
	for _, val := range arr {
		d := distanse(val.X, val.Y, P.X, P.Y)
		if d < P.R*1.8 {
			return true
		}
	}
	return res
}

func main() {
	p.Setup = setup
	p.Draw = draw

	p.LaunchApp()

}

func distanse(x1, y1, x2, y2 float64) float64 {
	return math.Sqrt((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1))
}
