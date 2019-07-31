package main

import (
	dft "PGgoJS/ParamFourierTrain/DiscreteFourierT"
	v "PGgoJS/ParamFourierTrain/vectorObj"
	"fmt"
	"math"
	"sort"

	p "github.com/bregydoc/PGoJs/Processing"
)

var x []float64
var y []float64

var fourierX []dft.ComplN
var fourierY []dft.ComplN

var inTime float64
var path []v.Vector

var width = 750
var height = 741

func setup() {
	p.CreateCanvas(width, height)
	p.Background(0)
	const skip = 6
	for i := 0; i < len(dft.Drawing); i += skip {
		x = append(x, dft.Drawing[i].X)
		y = append(y, dft.Drawing[i].Y)
	}

	// for i := 0.; i < 200*math.Pi; i += 0.1 {
	// 	x = append(x, 100*(2*math.Cos(i)+math.Sin(2*i)*math.Cos(60*i)))
	// 	y = append(y, 100*(math.Sin(2*i)+math.Sin(60*i)))
	// }

	// for i := 0.; i < 20*math.Pi; i += 0.1 {
	// 	x = append(x, 5*(math.Cos(i)+i*math.Sin(i)))
	// 	y = append(y, 5*(math.Sin(i)-i*math.Cos(i)))
	// }
	fourierX = dft.DFT(x)
	fourierY = dft.DFT(y)
	sort.Slice(fourierX, func(i, j int) bool { return dft.AbsC(fourierY[i]) > dft.AbsC(fourierY[j]) })
	sort.Slice(fourierY, func(i, j int) bool { return dft.AbsC(fourierY[i]) > dft.AbsC(fourierY[j]) })

	fmt.Println(fourierX)
}

func draw() {

	p.Background(0)
	p.Stroke(255)
	p.NoFill()
	p.Text(fmt.Sprintf("%.2f", inTime), 200, height-50)

	vx := epicycles(float64(width)/2+100, 100, 0, fourierX)
	vy := epicycles(100, float64(height)/2+100, math.Pi/2, fourierY)
	v := v.NewVector(vx.X, vy.Y)

	path = append(path, v)

	for i := len(path) - 1; i > 1; i-- {
		p.Line(path[i].X, path[i].Y, path[i-1].X, path[i-1].Y)
	}

	p.Line(vx.X, vx.Y, v.X, v.Y)
	p.Line(vy.X, vy.Y, v.X, v.Y)

	var Dt = 2 * math.Pi / float64(len(fourierY))
	inTime += Dt

	if inTime > 2*math.Pi {
		inTime = 0
		path = path[:0]
		// for i := 0; i < len(path); i++ {
		// 	del(&path, i)
		// }
	}

	// if len(path) > 800 {
	// 	del(&path, 0)
	// }
}

func main() {
	p.Setup = setup
	p.Draw = draw
	p.LaunchApp()

}

func epicycles(x, y, rotation float64, fourier []dft.ComplN) v.Vector {

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

	return v.NewVector(x, y)

}

func del(arr *[]v.Vector, n int) {

	slice := (*arr)
	for i := 0; i < len(*arr); i++ {
		if i == n {
			slice = slice[:i+copy(slice[i:], slice[i+1:])]
			*arr = slice
		}
	}

}
