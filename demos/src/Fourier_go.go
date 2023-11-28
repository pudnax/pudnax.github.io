package main

import (
	dft "PGgoJS/FourierDraw/DiscreteFourierT"
	vc "PGgoJS/FourierDraw/vectorObj"
	"fmt"
	"math"
	"sort"

	p "github.com/bregydoc/PGoJs/Processing"
)

var y []float64
var x []float64 = []float64{300}

var fourierY []dft.ComplN

var inTime float64
var path []vc.Vector
var vec []vc.Vector

var width = 1250
var height = 741

func setup() {
	p.CreateCanvas(width, height)
	p.Background(0)

	for i := 0.; i < 20*math.Pi; i += 0.1 {
		y = append(y, 100*math.Cos(i))
	}
	fourierY = dft.DFT(y)
	sort.Slice(fourierY, func(i, j int) bool { return dft.AbsC(fourierY[i]) < dft.AbsC(fourierY[j]) })
}

func draw() {

	x = append(x, x[len(x)-1]+1)
	p.Background(0)
	p.Stroke(255)
	p.NoFill()
	p.Text(fmt.Sprintf("%.2f", inTime), 100, height-50)

	vy := epicycles(100, float64(width)/2-400, math.Pi/2, fourierY)
	v := vc.NewVector(x[len(x)-1], vy.Y)

	path = append([]vc.Vector{v}, path...)

	// for i := len(path) - 1; i > 1; i-- {
	// 	p.Line(path[i].X, path[i].Y, path[i-1].X, path[i-1].Y)
	// }

	for i := 1; i < len(path); i++ {
		p.Line(300+float64(i), path[i].Y, 300+float64(i)-1, path[i-1].Y)
	}

	p.Line(vy.X, vy.Y, 300, path[0].Y)

	var Dt = 2 * math.Pi / float64(len(fourierY))
	inTime += Dt

	// if inTime > 2*math.Pi {
	// 	inTime = 0
	// 	path = path[:0]
	// 	x = x[:1]
	// 	// for i := 0; i < len(path); i++ {
	// 	// 	del(&path, i)
	// 	// }
	// }

	if len(path) > 800 {
		del(&path, len(path)-1)
	}
}

func main() {
	p.Setup = setup
	p.Draw = draw
	p.LaunchApp()

}

func epicycles(x, y, rotation float64, fourier []dft.ComplN) vc.Vector {

	for i := 0; i < len(fourierY); i++ {
		prevx := x
		prevy := y

		var frec = fourier[i].Frec
		var ampl = fourier[i].Ampl
		var phase = fourier[i].Phase
		x += ampl * math.Cos(frec*inTime+phase+rotation)
		y += ampl * math.Sin(frec*inTime+phase+rotation)

		p.Ellipse(prevx, prevy, ampl, ampl)
		p.Line(prevx, prevy, x, y)
		p.Fill(255)
		p.Ellipse(x, y, 2, 2)
		p.NoFill()

	}

	return vc.NewVector(x, y)

}

func del(arr *[]vc.Vector, n int) {

	slice := (*arr)
	for i := 0; i < len(*arr); i++ {
		if i == n {
			slice = slice[:i+copy(slice[i:], slice[i+1:])]
			*arr = slice
		}
	}

}
