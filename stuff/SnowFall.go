package main

import (
	. "PGgoJS/snowfall/vectorObj"
	"fmt"
	"math"
	"math/rand"

	gp "github.com/aquilax/go-perlin"
	p "github.com/bregydoc/PGoJs/Processing"
)

var widht = 1535.
var height = 741.
var snow []*Snowflake
var v = Vector{}
var gravity = NewVector(0, 4)
var sourc = rand.NewSource(rand.Int63())
var perl = gp.NewPerlinRandSource(2, 2, 1, sourc)
var zOff float64

func setup() {
	p.CreateCanvas(int(widht), int(height))
	p.Background(0)
	// p.FullScreen()

	// for i := 0; i < 900; i++ {
	// 	snow = append(snow, newFlake())
	// }
}

func draw() {
	p.Background(0)
	p.Stroke(255)
	// p.Fill(255)

	wx := func(mx int, min, max, start, end float64) float64 {
		val := float64(mx)
		val = constrain(val, min, max)
		val /= max
		val -= 0.5
		val *= (math.Abs(start) + math.Abs(end)) / 2
		return val
	}(p.MouseX, 0, widht, -4, 4)
	// wx := float64(p.MouseX)
	mwind := NewVector(wx, 0)
	zOff += 0.02

	snow = append(snow, newFlake())
	for _, flake := range snow {
		xOff := flake.pos.X / widht
		yOff := flake.pos.Y / height

		wAngle := perl.Noise3D(xOff, yOff, zOff) * 2 * math.Pi
		wind := FromAngle(wAngle)
		wind.Mult(1.2)

		flake.applyForce(gravity)
		flake.applyForce(wind)
		flake.applyForce(mwind)

		flake.update()
		flake.render()
	}

	for i := len(snow) - 1; i >= 0; i-- {
		if snow[i].offScreen() {
			del(&snow, i)
		}
	}
	fmt.Println(len(snow), wx)
}

func getRandomSize() int {
	// r := math.Pow((rand.Float64()), 18)
	// return int(constrain(r*10, 5, 50))

	r := rand.NormFloat64() * 3
	return int(constrain(math.Abs(r), 1, 36))

	// for {
	// 	r1 := rand.Float64()
	// 	r2 := rand.Float64()
	// 	if r2 > r1 {
	// 		return int(r1 * 10)
	// 	}
	// }
}

func newFlake() *Snowflake {
	x := rand.Float64() * widht
	y := rand.Float64()*(-50) - 10
	chance := rand.Float64()
	text := ""
	if chance > 0.99 {
		text = "Нубас"
	}

	return &Snowflake{pos: NewVector(x, y),
		vel:   NewVector(0, 5),
		acc:   NewVector(),
		r:     getRandomSize(),
		angle: rand.Float64() * 2 * math.Pi,
		dir:   rand.Intn(2) - 1,
		text:  text,
	}
}

// Snowflake contain Christmas spirit
type Snowflake struct {
	pos, vel, acc     Vector
	r, dir            int
	mass, angle, xOff float64
	text              string
}

func (s *Snowflake) randomize() {
	x := rand.Float64() * widht
	y := rand.Float64()*(-50) - 10
	s.pos = NewVector(x, y)
	s.vel = NewVector(0, 5)
	s.acc = NewVector()
	s.r = getRandomSize()

}

func (s *Snowflake) offScreen() bool {
	return s.pos.Y > height+float64(s.r)
}

func (s *Snowflake) applyForce(force Vector) {
	//   Parallax Effect hack
	sample := NewVector(0, float64(s.r)/2)
	sample.Summ(5)
	s.acc.Add(Mult(force, sample))
}

func (s *Snowflake) update() {
	s.xOff = math.Sin(s.angle/200) * 8 * float64(s.r) * float64(s.dir)

	s.vel.Add(s.acc)
	s.vel.Limit(float64(s.r) * 0.08)

	if s.vel.Magn*s.vel.Magn < 1 {
		s.vel.Normalize()
	}

	s.pos.Add(s.vel)
	s.acc.Mult(0)

	// if s.pos.Y > height+float64(s.r) {
	// 	s.randomize()
	// }

	if s.pos.X < -float64(s.r) {
		s.pos.X = widht + float64(s.r)
	}
	if s.pos.X > widht+float64(s.r) {
		s.pos.X = -float64(s.r)
	}
}

func (s *Snowflake) render() {
	p.Stroke(255)
	p.StrokeWeight(s.r)
	p.Point(s.pos.X+s.xOff, s.pos.Y)

	// p.Text(s.text, s.pos.X+s.xOff, s.pos.Y)

	s.angle += s.vel.MagnSq()
}

func main() {
	p.Setup = setup
	p.Draw = draw

	p.LaunchApp()

}

func distanse(x1, y1, x2, y2 float64) float64 {
	return math.Sqrt((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1))
}

func del(arr *[]*Snowflake, n int) {

	slice := (*arr)
	for i := 0; i < len(*arr); i++ {
		if i == n {
			slice = slice[:i+copy(slice[i:], slice[i+1:])]
			*arr = slice
		}
	}

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
