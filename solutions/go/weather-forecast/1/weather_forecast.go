// Package weather is a package for weathher broadcast.
package weather

// CurrentCondition is a current value of condition.
var CurrentCondition string

// CurrentLocation is a current value of location.
var CurrentLocation string

// Forecast helps you forecast current weather.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
