package main

import (
	dft "PGgoJS/CustomFourier/DiscreteFourierT"
	v "PGgoJS/CustomFourier/vectorObj"
	"fmt"
	"math"
	"math/cmplx"
	"sort"

	p "github.com/bregydoc/PGoJs/Processing"
)

var state bool = true

var signal []complex128
var drawing = []v.Vector{v.NewVector(0, 0)}

var fourier []dft.CmplxN

var inTime float64
var path []v.Vector

var width = 750
var height = 741

func mousePressed() {
	state = true
	path = path[:0]
	drawing = drawing[:0]
	fourier = fourier[:0]
	signal = signal[:0]
	fmt.Println(path)
}

func mouseReleased() {
	state = false
	for i := 0; i < len(drawing); i++ {
		signal = append(signal, complex(drawing[i].X, drawing[i].Y))
	}
	fourier = dft.DFT(signal)
	sort.Slice(fourier, func(i, j int) bool { return cmplx.Abs(fourier[i].Wave) > cmplx.Abs(fourier[j].Wave) })

}

func setup() {
	p.CreateCanvas(width, height)
	p.Background(0)

	// for i := 0.; i < 200*math.Pi; i += 0.1 {
	// 	x = append(x, 100*(2*math.Cos(i)+math.Sin(2*i)*math.Cos(60*i)))
	// 	y = append(y, 100*(math.Sin(2*i)+math.Sin(60*i)))
	// }

	// for i := 0.; i < 20*math.Pi; i += 0.1 {
	// 	x = append(x, 5*(math.Cos(i)+i*math.Sin(i)))
	// 	y = append(y, 5*(math.Sin(i)-i*math.Cos(i)))
	// }
}

func draw() {
	p.Background(0)
	p.Stroke(255)
	p.NoFill()
	p.Text(fmt.Sprintf("%.2f", inTime), 100, height-50)

	if state == true {
		stalker := v.NewVector(float64(p.MouseX)-float64(width)/2, float64(p.MouseY)-float64(height)/2)
		drawing = append(drawing, stalker)
		// compare := drawing[len(drawing)-2]
		// if stalker != compare {
		// 	del(&drawing, len(drawing)-1)
		// }
	} else if state == false {

		epic := epicycles(float64(width)/2, float64(height)/2, 0, fourier)
		v := v.NewVector(epic.X, epic.Y)

		path = append(path, v)

		for i := len(path) - 1; i > 1; i-- {
			p.Line(path[i].X, path[i].Y, path[i-1].X, path[i-1].Y)
		}

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
	var Dt = 2 * math.Pi / float64(len(fourier))
	inTime += Dt
}

func main() {
	p.Setup = setup
	p.Draw = draw
	p.MousePressed = mousePressed
	p.MouseReleased = mouseReleased
	p.LaunchApp()

}

func epicycles(x, y, rotation float64, fourier []dft.CmplxN) v.Vector {

	for i := 0; i < len(fourier); i++ {
		prevx := x
		prevy := y

		var frec = fourier[i].Frec
		ampl, phase := cmplx.Polar(fourier[i].Wave)
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
