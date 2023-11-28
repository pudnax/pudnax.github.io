package main

import (
	"fmt"
	"math"

	p "github.com/bregydoc/PGoJs/Processing"
)

var widht = 1535.
var height = 741.

var a = Vector{x: -300, y: -200}
var b = Vector{x: 300, y: -200}

var lenght = distanse(a.x, a.y, b.x, b.y)
var h = lenght * math.Sqrt(3) / 2
var c = Vector{x: 0, y: b.y + h}

var s1 = newSegment(a, b)
var s2 = newSegment(b, c)
var s3 = newSegment(c, a)

var segments []Segment

// var start = newSegment(a, b)

// var children = start.generate()

func setup() {
	p.CreateCanvas(int(widht), int(height))
	p.Background(0)
	// p.FullScreen()

	// segments = append(segments, children...)
	segments = append(segments, s1, s2, s3)
}

func mousePressed() {
	var nextGeneration []Segment

	for i := range segments {
		children := segments[i].generate()
		nextGeneration = append(nextGeneration, children...)
	}
	segments = nextGeneration
}

func draw() {
	p.Translate(widht/2, height/2)
	p.Background(0)
	p.Stroke(255)

	for i := range segments {
		segments[i].show()
	}

}

type Segment struct {
	a, b Vector
}

func newSegment(start, end Vector) Segment {
	return Segment{a: start, b: end}
}

func (s *Segment) generate() (children []Segment) {
	children = make([]Segment, 6)
	val := sub(s.a, s.b)
	val.div(3)
	// #1
	b1 := add(s.a, val)
	children[0] = newSegment(s.a, b1)

	// #2
	val.rotate(-math.Pi / 3)
	c1 := add(b1, val)
	children[1] = newSegment(b1, c1)

	// #3
	val.rotate(2 * math.Pi / 3)
	a2 := add(c1, val)
	children[2] = newSegment(c1, a2)
	// #4
	children[3] = newSegment(a2, s.b)
	// #5
	c2 := add(b1, val)
	children[4] = newSegment(b1, c2)
	// #6
	children[5] = newSegment(c2, a2)
	return
}

func (s Segment) show() {
	p.Stroke(255)
	p.Line(s.a.x, s.a.y, s.b.x, s.b.y)
}

func sub(a, b Vector) Vector {
	return Vector{x: b.x - a.x, y: b.y - a.y}
}
func add(a, b Vector) Vector {
	return Vector{x: b.x + a.x, y: b.y + a.y}
}

type Vector struct {
	x, y float64
}

func (v *Vector) div(val float64) {
	v.x /= val
	v.y /= val
}
func (v *Vector) rotate(angle float64) {
	v.x, v.y = math.Cos(angle)*v.x-math.Sin(angle)*v.y, math.Sin(angle)*v.x+math.Cos(angle)*v.y
}

func main() {
	fmt.Println(a.x, a.y, b.x, b.y)
	fmt.Println(segments)

	p.Setup = setup
	p.Draw = draw
	p.MousePressed = mousePressed

	p.LaunchApp()

}

func distanse(x1, y1, x2, y2 float64) float64 {
	return math.Sqrt((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1))
}
