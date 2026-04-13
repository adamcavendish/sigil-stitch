package main

func classify(x int) string {
	if x > 0 {
		return "positive"
	}
	if x < 0 {
		return "negative"
	}
	return "zero"
}
