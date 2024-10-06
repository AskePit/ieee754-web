import init, { decimal_to_binary, binary_to_decimal, binary_to_decimal_ext, get_predefined_layout, PredefinedLayout } from './ieee754_web.js';

await init();

const DECIMAL_PRECISION = 20

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

const signBitText = document.getElementById('sign-bit-text')
const exponentBitsText = document.getElementById('exponent-bits-text')
const mantissaBitsText = document.getElementById('mantissa-bits-text')

const normalizedLabel = document.getElementById('normalized-section')

// dec edit
decInputField.oninput = () => {
    let filtered = ''
    for(let c of decInputField.value) {
        if (c >= '0' && c <= '9') {
            filtered += c
        } else if (c == '.' || c == ',' || c == '-') {
            filtered += c
        }
    }

    decInputField.value = filtered
}

// dec submit
decInputField.onchange = () => {
    if (!decInputField.value.includes('NaN') && !decInputField.value.includes('Infinity')) {
        if (decInputField.value.length == 0) {
            decInputField.value = "0"
        }
        if(decInputField.value.includes(',')) {
            decInputField.value.replace(',', '.')
        }
        if (!decInputField.value.includes('.')) {
            decInputField.value += '.0'
        }
    }
    SetDataFromDec()
}

for(let binInputField of binInputFields) {
    // bin edit
    binInputField.oninput = () => {
        let filtered = ''
        for(let i = 0; i<4; ++i) {
            const c = binInputField.value[i]
            if (c >= '0' && c <= '1') {
                filtered += c
            }
        }
    
        binInputField.value = filtered
    }

    // bin submit
    binInputField.onchange = () => {
        if (binInputField.value.length < 4) {
            binInputField.value = '0'.repeat(4 - binInputField.value.length) + binInputField.value
        }
        SetDataFromBin()
    }
}

for(let hexInputField of hexInputFields) {
    // hex edit
    hexInputField.oninput = () => {
        let filtered = ''
        for(let i = 0; i<4; ++i) {
            const c = hexInputField.value[i]
            if (c >= '0' && c <= '9') {
                filtered += c
            } else if (c >= 'A' && c <= 'F') {
                filtered += c
            } else if (c >= 'a' && c <= 'f') {
                filtered += c
            }
        }

        hexInputField.value = filtered
    }

    // hex submit
    hexInputField.onchange = () => {
        if (hexInputField.value.length < 4) {
            hexInputField.value = '0'.repeat(4 - hexInputField.value.length) + hexInputField.value
        }
        hexInputField.value = hexInputField.value.toUpperCase()
        SetDataFromHex()
    }
}

copyDecButton.onclick = () => {
    navigator.clipboard.writeText(decInputField.value)
}

copyBinButton.onclick = () => {
    navigator.clipboard.writeText(GetBitsFromBin())
}

copyHexButton.onclick = () => {
    navigator.clipboard.writeText(hexInputFields[0].value + hexInputFields[1].value)
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

function SetBitsToLabels(bits) {
    const info = binary_to_decimal_ext(bits, get_predefined_layout(PredefinedLayout.Float32), DECIMAL_PRECISION)
    signBitText.innerHTML = info.is_positive ? '+' : '-'

    if (info.are_exponent_and_mantissa_valid) {
        exponentBitsText.innerHTML = '2<sup>' + info.exponent + '</sup>'
        mantissaBitsText.innerHTML = info.mantissa
    } else {
        exponentBitsText.innerHTML = ''
        mantissaBitsText.innerHTML = ''
    }
    
    normalizedLabel.innerHTML = '<i>' + (info.is_denormalized ? 'denormalized' : 'normalized') + '</i>'
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
        hexInputFields[i].value = hex.slice(i*4, i*4 + 4).toUpperCase()
    }
}

function GetBitsFromDec() {
    return decimal_to_binary(decInputField.value, get_predefined_layout(PredefinedLayout.Float32))
}

function SetBitsToDec(bits) {
    let val = binary_to_decimal(bits, get_predefined_layout(PredefinedLayout.Float32), DECIMAL_PRECISION)
    if (!val.includes('NaN') && !val.includes('Infinity')) {
        if (!val.includes('.') && !val.includes(',')) {
            val += '.0'
        }
    }
    decInputField.value = val
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
        SetBitsToCheckboxes,
        SetBitsToLabels
    ])
}

function SetDataFromCheckboxes() {
    SetData(GetBitsFromCheckboxes(), [
        SetBitsToDec,
        SetBitsToHex,
        SetBitsToBin,
        SetBitsToLabels
    ])
}

function SetDataFromDec() {
    SetData(GetBitsFromDec(), [
        SetBitsToHex,
        SetBitsToBin,
        SetBitsToCheckboxes,
        SetBitsToLabels
    ])
}

function SetDataFromBin() {
    SetData(GetBitsFromBin(), [
        SetBitsToDec,
        SetBitsToHex,
        SetBitsToCheckboxes,
        SetBitsToLabels
    ])
}

function SetDataFromHex() {
    SetData(GetBitsFromHex(), [
        SetBitsToDec,
        SetBitsToBin,
        SetBitsToCheckboxes,
        SetBitsToLabels
    ])
}

function SetZero() {
    SetAllData('00000000000000000000000000000000')
}

function SetInf() {
    SetAllData('01111111100000000000000000000000')
}

function SetNan() {
    SetAllData('01111111110000000000000000000001')
}

SetAllData('00111111100000000000000000000000')