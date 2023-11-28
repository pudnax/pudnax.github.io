package main

import (
	"fmt"
	"math"
	v "vectorObj"

	p "github.com/bregydoc/PGoJs/Processing"
)

var inTime float64
var width = 850
var height = 850
var radius = float64((width+height)/4 - 50)

var factor = 0.001
var total = 200.

func newCardioid(n float64) cardioid {
	av := make([]v.Vector, int(n))
	bv := make([]v.Vector, int(n))
	for i := 0; i < int(n); i++ {
		angle := mapp(float64(i), 0, total, 0, 2*math.Pi)
		a := v.FromAngle(angle + math.Pi)
		b := v.FromAngle(angle*factor + math.Pi)
		a.Mult(radius)
		b.Mult(radius)
		av = append(av, a)
		bv = append(bv, b)
	}
	return cardioid{N: n, a: av, b: bv}
}

type cardioid struct {
	a, b []v.Vector
	N    float64
}

func (c cardioid) draw() {
	for i := range c.b {
		p.Ellipse(c.b[i].X, c.b[i].Y, 8, 8)
		p.Line(c.a[i].X, c.a[i].Y, c.b[i].X, c.b[i].Y)
	}
	// for i := 0; i < int(c.N); i++ {
	// 	p.Ellipse(c.b[i].X, c.b[i].Y, 8, 8)
	// }

	// for i := 0; i < int(c.N); i++ {
	// 	a := v.FromAngle(float64(i) + math.Pi)
	// 	b := v.FromAngle(float64(i)*factor + math.Pi)
	// 	a.Mult(radius)
	// 	b.Mult(radius)
	// 	p.Line(a.X, a.Y, b.X, b.Y)
	// }
}

func setup() {
	p.CreateCanvas(width, height)
	p.Background(0)
}
func draw() {
	p.Translate(width/2, height/2)
	p.Background(0)
	p.NoFill()
	p.Stroke(255)
	p.Ellipse(0, 0, radius*2, radius*2)
	p.Fill(255)

	card := newCardioid(total)
	card.draw()

	factor += 0.005

	if factor == 100. {
		factor = 0.1
	}
}

func main() {
	p.Setup = setup
	p.Draw = draw

	p.LaunchApp()
	for i := 0; i < 10; i++ {
		fmt.Println(mapp(float64(i), 0, 10, 50, 100))
	}
}

func mapp(val float64, start, stop float64, firstOfRange, lastOfRange float64) float64 {
	delta := stop - start
	return firstOfRange + (lastOfRange-firstOfRange)/float64(delta)*val
}

func nextVec(n float64) v.Vector {
	return v.NewVector(0, 0)
}
