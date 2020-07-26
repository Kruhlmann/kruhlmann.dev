/** Cookie interface. */
export class Cookie {
    private _name: string;
    private _value: string;
    private _days_dur: number;
    private _expiration_date: Date | undefined;
    public on_change: ((value: string | number) => void) | undefined;

    /**
     * Constructor.
     *
     * @param name - Cookie identifier.
     * @param value - Value of cookie. If not set will use Cookie.read to find
     * the existing value of the cookie with name
     * equal to the passed name. If the cookie is not found value will be an
     * empty string.
     * @param days_dur - Lifespan in days. If set to less than or
     * equal to 0 will set the lifespan to that of the session.
     * @returns
     */
    constructor(name: string, value?: string | number, days_dur = 0) {
        this._name = name;
        // If value is set then simply assign it to this.value. If no value was
        // provided load it from the cookie string. If all else fails simply set
        // the value to an empty string.
        if (value) {
            this._value = `${value}`;
        } else {
            const cookie_string = new RegExp(`${name}[^;]+`).exec(
                document.cookie,
            );
            this._value = decodeURIComponent(
                cookie_string
                    ? cookie_string.toString().replace(/^[^=]+./, "")
                    : "",
            );
        }
        this._days_dur = days_dur;
        if (days_dur > 0) {
            const exp_date = new Date();
            exp_date.setTime(
                exp_date.getTime() + days_dur * 24 * 60 * 60 * 1000,
            );
            this._expiration_date = exp_date;
        }
    }

    /**
     * Gets and sets the value of the cookie.
     *
     * @param value - Value to set.
     * @returns - Cookie value if no value was provided, otherwise sets
     * the value to the new value and returns an empty string.
     */
    public val(value?: string | number): string {
        if (value) {
            this._value = `${value}`;
            this.attach();
            if (typeof this.on_change === "function") {
                this.on_change(value);
            }
        }
        return this._value;
    }

    /**
     * Attatches the cookie to the document.
     *
     * @returns - This object for chaining functions.
     */
    public attach(): Cookie {
        let expiration = "";

        if (this._expiration_date) {
            expiration = `; expires=${this._expiration_date.toUTCString()}`;
        }

        const cookie_key_value = `${this._name}=${this._value || ""}`;
        const cookie_suffix = "; path=/;";
        const cookie_string = cookie_key_value + expiration + cookie_suffix;

        document.cookie = cookie_string;
        return this;
    }

    /**
     * Removes the cookie from the document.
     *
     * @returns This object for chaining functions.
     */
    public remove(): Cookie {
        document.cookie = `${this._name}=; Max-Age=-99999999`;
        return this;
    }

    /**
     * Updates the expiration date of the cookie and sets it equal to the
     * current date plus the pre-defined lifespan in days.
     *
     * @returns This object for chaining functions.
     */
    public refresh(): Cookie {
        if (this._days_dur > 0) {
            const exp_date = new Date();
            const exp_time = exp_date.getTime();
            exp_date.setTime(exp_time + this._days_dur * 24 * 60 * 60 * 1000);
            this._expiration_date = exp_date;
        } else {
            this._expiration_date = undefined;
        }
        this.attach();
        return this;
    }

    /**
     * Override the toString default method from Object.
     *
     * @override
     * @returns - Value of cookie.
     */
    public toString(): string {
        return this._value;
    }
}
