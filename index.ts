const {text_to_binary, binary_to_text, decimal_to_binary}  = require("./index.node")

export class BinaryStreaming{
    textToBinary(text:string):string{
        return text_to_binary(text)
    }

    binaryToText(binaryText:string):string{
        return binary_to_text(binaryText)
    }

    decimalToBinary(decimal:number):string{
        return decimal_to_binary(decimal)
    }
}