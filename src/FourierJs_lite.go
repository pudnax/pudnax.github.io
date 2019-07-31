package main

import (
	"fmt"
	"math"

	p "github.com/bregydoc/PGoJs/Processing"
)

var inTime float64

func setup() {
	// slider := p.CreateSlider(1, 5)
	// slider.Position(200, 500)
	// p.CreSlider(1, 5)
	p.CreateCanvas(1532, 741)
	p.Background(0)
}
func draw() {

	p.Background(0)
	var wave [1000]float64
	p.Stroke(255)
	p.NoFill()

	x := 0.
	y := 0.
	N := 40
	for i := 0; i < N; i++ {
		prevx := x
		prevy := y
		p.Text(fmt.Sprintf("%.2f", inTime), 200, 741-50)
		muzzD(200, 541-50)
		muzzU(400, 541-50)
		muzzU(200, 200)

		n := float64(2*i + 1)
		// n := (float64(i + 1))

		// var ampl = 400 * math.Pow(-1, (n-1)/2) / (n * n)
		// var ampl = float64(-400 / (math.Pi * (1-math.Pow(n, 2))))
		var ampl = float64(200 * (4 / (n * math.Pi)))

		x += ampl / 2. * math.Cos(float64(n)*inTime)
		y += ampl / 2. * math.Sin(float64(n)*inTime)

		p.Ellipse(200+prevx, 200+prevy, ampl, ampl)
		p.Line(200+prevx, 200+prevy, 200+x, 200+y)
		p.Fill(255)
		p.Ellipse(200+x, 200+y, 8, 8)
		p.NoFill()

		if i == 0 {
			p.Text("Киселев", 200+x, 215+y)
		}
		if i == N-1 {
			p.Text("то, как он нами крутит", 200+x, 215+y)
		}

		wave = func(time float64, arr [1000]float64) [1000]float64 {
			for j := 0; j < len(arr); j++ {
				arr[j] += ampl / 2. * math.Sin(n*(inTime-0.02*float64(j)))
			}
			return arr
		}(inTime, wave)
	}
	//
	kk := 80
	p.Line(200+x, 200+y, 500+kk, 200+wave[kk])
	//
	p.Line(200+x, 200+y, 500, 200+wave[0])

	for i := 0; i < len(wave); i++ {
		p.Point(500+float64(i), 200+wave[i])
	}

	// for step := 0; step < 4; step++ {
	// 	muzzU(600+100*float64(step), 250)
	// 	muzzU(600+200*float64(step), 250)
	// }

	inTime += 0.03
	//
	fmt.Println(wave)
}

func muzzD(x, y float64) {
	p.Line(x, y-7, x+15, y+15)
	p.Line(x, y-7, x-15, y+15)

	p.Line(x+30, y+10, x+75, y+25)
	p.Line(x+35, y, x+80, y)
	p.Line(x+30, y-10, x+75, y-25)

	p.Line(x-30, y+10, x-75, y+25)
	p.Line(x-35, y, x-80, y)
	p.Line(x-30, y-10, x-75, y-25)

	p.Line(x+20, y+30, x+50, y+40)
	p.Line(x-20, y+30, x-50, y+40)
}

func muzzU(x, y float64) {
	p.Line(x, y+7, x+15, y-15)
	p.Line(x, y+7, x-15, y-15)

	p.Line(x+30, y+10, x+75, y+25)
	p.Line(x+35, y, x+80, y)
	p.Line(x+30, y-10, x+75, y-25)

	p.Line(x-30, y+10, x-75, y+25)
	p.Line(x-35, y, x-80, y)
	p.Line(x-30, y-10, x-75, y-25)

	p.Line(x+20, y-30, x+50, y-40)
	p.Line(x-20, y-30, x-50, y-40)
}

func mask(arr [1000]float64, key func(float64) bool) [1000]bool {
	var res [1000]bool
	for i := 0; i < len(arr); i++ {
		res[i] = key(arr[i])
	}
	return res
}

func main() {
	p.Setup = setup
	p.Draw = draw

	p.LaunchApp()

}
