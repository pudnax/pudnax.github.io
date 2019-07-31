package main

import (
	"fmt"
	v "vectorObj"

	p "github.com/bregydoc/PGoJs/Processing"
)

var inTime float64
var width = 1500
var height = 750
var spd = v.NewVector(2, 2)
var box = newBox(400, 300)

func setup() {
	p.CreateCanvas(width, height)
	p.Background(0)
}
func draw() {
	p.Background(0)

	box.point.Add(box.speed)

	box.ifCollision()
	box.update()
	fmt.Println(box.speed, box.point)
}

func newBox(x, y float64) Box {
	return Box{point: v.NewVector(x, y), speed: spd, w: 10, h: 6}
}

type Box struct {
	point, speed v.Vector
	w, h         float64
}

func (b *Box) update() {
	p.Rect(b.point.X, b.point.Y, b.w, b.h)
}

func (b *Box) ifCollision() {

	if b.point.X <= 0 {
		b.point.X = 0
		b.speed.X *= -1
	} else if b.point.X+b.w >= float64(width) {
		b.point.X = float64(width) - b.w
		b.speed.X *= -1
	}

	if b.point.Y <= 0 {
		b.point.Y = 0
		b.speed.Y *= -1
	} else if b.point.Y+b.h >= float64(height) {
		b.point.Y = float64(height) - b.h
		b.speed.Y *= -1
	}
}

func main() {
	p.Setup = setup
	p.Draw = draw

	p.LaunchApp()
}
