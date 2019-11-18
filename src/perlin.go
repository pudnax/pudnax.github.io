package main

import (
	"math"
	"math/rand"
	"time"

	gp "github.com/ojrac/opensimplex-go"

	p "github.com/bregydoc/PGoJs/Processing"
)

var width = 750
var height = 750
var sourc = rand.NewSource(rand.Int63())
var zoff float64
var phase float64
var perl = gp.NewNormalized(sourc.Int63())

func setup() {
	p.CreateCanvas(width, height)
	p.Background(0)
	rand.Seed(time.Now().UTC().UnixNano())
}
func draw() {
	p.Background(0)
	p.Stroke(255)
	p.Translate(width/2, height/2)

	var inTime float64
	for a := 0.; a < 2*math.Pi; a += 0.005 {
		// r := 100 + 130.*rand.Float64()
		var xoff = math.Cos(a + phase)
		var yoff = math.Sin(a + phase)
		r := 250 * perl.Eval3(xoff, yoff, zoff)
		x := r * math.Cos(a)
		y := r * math.Sin(a)
		p.Point(x, y)

		inTime += 0.05

	}
	phase += 0.003
	zoff += 0.01
}

func main() {
	p.Setup = setup
	p.Draw = draw

	p.LaunchApp()
}
