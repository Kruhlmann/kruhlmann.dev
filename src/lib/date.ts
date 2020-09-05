function pad_number(number_to_pad: number): string {
    if (number_to_pad < 10) {
        return `0${number_to_pad}`;
    }
    return number_to_pad.toString();
}

export function date_to_timestamp(date: Date): string {
    const hours = pad_number(date.getHours());
    const minutes = pad_number(date.getMinutes());
    const seconds = pad_number(date.getSeconds());
    return `${hours}:${minutes}:${seconds}`;
}
