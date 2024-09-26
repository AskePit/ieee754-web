import init, { float32_to_binary, binary_to_float32 } from './ieee754_web.js';

await init();

const decInputField = document.getElementById('dec-input-field')
const binInputFields = document.getElementsByClassName('bin-input-field')
const hexInputFields = document.getElementsByClassName('hex-input-field')

const copyDecButton = document.getElementById('button-copy-dec')
const copyBinButton = document.getElementById('button-copy-bin')
const copyHexButton = document.getElementById('button-copy-hex')

const zeroButton = document.getElementById('button-zero')
const infButton = document.getElementById('button-inf')
const nanButton = document.getElementById('button-nan')

const signBit = document.getElementById('sign-bit')
const exponentBits = document.getElementsByClassName('exponent-bit')
const mantissaBits = document.getElementsByClassName('mantissa-bit')

const normalizedLabel = document.getElementById('normalized-section')

decInputField.onchange = () => {
    SetDataFromDec()
}

for(let binInputField of binInputFields) {
    binInputField.onchange = () => {
        SetDataFromBin()
    }
}

for(let hexInputField of hexInputFields) {
    hexInputField.onchange = () => {
        SetDataFromHex()
    }
}

copyDecButton.onclick = () => {
}

copyBinButton.onclick = () => {
}

copyHexButton.onclick = () => {
}

zeroButton.onclick = () => {
    SetZero()
}

infButton.onclick = () => {
    SetInf()
}

nanButton.onclick = () => {
    SetNan()
}

signBit.onclick = () => {
    SetDataFromCheckboxes()
}

for (let i = 0; i<exponentBits.length; ++i) {
    const exponentBit = exponentBits[i]
    exponentBit.onclick = () => {
        SetDataFromCheckboxes()
    }
}

for (let i = 0; i<mantissaBits.length; ++i) {
    const mantissaBit = mantissaBits[i]
    mantissaBit.onclick = () => {
        SetDataFromCheckboxes()
    }
}

function GetBitsFromCheckboxes() {
    let bits = signBit.checked ? '1' : '0'

    for (const exponentBit of exponentBits) {
        bits += exponentBit.checked ? '1' : '0'
    }

    for (const mantissaBit of mantissaBits) {
        bits += mantissaBit.checked ? '1' : '0'
    }

    return bits
}

function SetBitsToCheckboxes(bits) {
    signBit.checked = bits[0] == '1'

    for (let i = 0; i<exponentBits.length; ++i) {
        const exponentBit = exponentBits[i]
        exponentBit.checked = bits[i + 1] == '1'
    }

    for (let i = 0; i<mantissaBits.length; ++i) {
        const mantissaBit = mantissaBits[i]
        mantissaBit.checked = bits[i + 1 + 8] == '1'
    }
}

function GetBitsFromBin() {
    let bits = ''

    for (const binInputField of binInputFields) {
        bits += binInputField.value
    }
    
    return bits
}

function SetBitsToBin(bits) {
    for (let i = 0; i<8; ++i) {
        binInputFields[i].value = bits.slice(i*4, i*4 + 4)
    }
}

function GetBitsFromHex() {
    let hex = hexInputFields[0].value + hexInputFields[1].value
    return hex2bin(hex)
}

function SetBitsToHex(bits) {
    let hex = bin2hex(bits)

    for (let i = 0; i<2; ++i) {
        hexInputFields[i].value = hex.slice(i*4, i*4 + 4)
    }
}

function GetBitsFromDec() {
    return float32_to_binary(decInputField.value)
}

function SetBitsToDec(bits) {
    decInputField.value = binary_to_float32(bits)
}

function hex2bin(hex) {
    return (parseInt(hex, 16).toString(2)).padStart(32, '0');
}

function bin2hex(bin) {
    return (parseInt(bin, 2).toString(16)).padStart(8, '0');
}

function SetData(bits, setFunctionsList) {
    for(const f of setFunctionsList) {
        f(bits)
    }
}

function SetAllData(bits) {
    SetData(bits, [
        SetBitsToDec,
        SetBitsToBin,
        SetBitsToHex,
        SetBitsToCheckboxes
    ])
}

function SetDataFromCheckboxes() {
    SetData(GetBitsFromCheckboxes(), [
        SetBitsToDec,
        SetBitsToHex,
        SetBitsToBin
    ])
}

function SetDataFromDec() {
    SetData(GetBitsFromDec(), [
        SetBitsToHex,
        SetBitsToBin,
        SetBitsToCheckboxes
    ])
}

function SetDataFromBin() {
    SetData(GetBitsFromBin(), [
        SetBitsToDec,
        SetBitsToHex,
        SetBitsToCheckboxes
    ])
}

function SetDataFromHex() {
    SetData(GetBitsFromHex(), [
        SetBitsToDec,
        SetBitsToBin,
        SetBitsToCheckboxes
    ])
}

function SetZero() {
    SetAllData('00000000000000000000000000000000')
}

function SetInf() {
    SetAllData('01111111100000000000000000000000')
}

function SetNan() {
    SetAllData('01111111110000000000000000000000')
}