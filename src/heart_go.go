package main

import (
	"math"

	p "github.com/bregydoc/PGoJs/Processing"
)

var inTime float64 = -100
var width = 800
var height = 800

var vect [][2]float64
var phi = 0.
var a = 0.

func setup() {
	p.CreateCanvas(width, height)
	p.Background(0)
	vect = append(vect, [2]float64{0, 0})
}

func draw() {
	p.Background(0)
	p.Stroke(255)
	p.Translate(width/2, height/2)
	var r = 80.
	// x := 16 * r * math.Sin(phi) * math.Sin(phi) * math.Sin(phi)
	// y := -r * (13*math.Cos(phi) - 5*math.Cos(2*phi) - 2*math.Cos(3*phi) - math.Cos(4*phi))
	add := 0.1
	x := r * (1 - math.Sin(phi)) * math.Cos(phi)
	y := -r*(1-math.Sin(phi))*math.Sin(phi) - 50
	vect = append(vect, [2]float64{x, y})
	phi += 0.1 + float64(a)
	p.Fill(255)
	easing := func(x float64) float64 {
		return math.Pow(math.Cos(float64(x)), 2) * 256
	}
	for i := 1; i < len(vect)-1; i++ {
		p.Stroke(p.Color(easing(float64(i)), easing(1-float64(i)), 255))

		p.Line(vect[i][0], vect[i][1], vect[i+1][0], vect[i+1][1])
	}
	if check := int(phi / (2 * math.Pi)); check == 1 {
		a += add
		add *= -1
	}
	p.TextSize(24)
	p.Stroke(constrain(inTime, 0, 255))
	p.Fill(constrain(inTime, 0, 255))
	p.Text("Мама", -40, 0)
	// if phi > 2*math.Pi {
	// 	phi = 0
	// 	vect = vect[:0]
	// }

	inTime += 0.7

}

func main() {
	p.Setup = setup
	p.Draw = draw

	p.LaunchApp()
}

func constrain(val, a, b float64) float64 {
	if val > b {
		return b
	}
	if val < a {
		return a
	}
	return val
}
