package main

import (
	"fmt"
	"math"
	"syscall/js"
)

var uint8Array = js.Global().Get("Uint8Array")

func multiply(this js.Value, args []js.Value) interface{} {
	array := args[0]
	length := array.Get("length").Int()

	// fmt.Println("JS array: ", length, array.Call("toString"))

	data := make([]byte,length)
	values := make([]byte, length)

	js.CopyBytesToGo(values, array)

	// fmt.Println(values)

	for index, element := range values {
    	data[index] = math.Exp(element)
	}

	buf := uint8Array.New(length)
	js.CopyBytesToJS(buf, data)
	
	return buf
}


func main() {
	fmt.Println("Hello, WebAssembly!")
	done := make(chan struct{}, 0)

	multiplyFunc := js.FuncOf(multiply) // wrapper function
	js.Global().Set("multiply", multiplyFunc)

	defer multiplyFunc.Release()
	<-done
}