const MS_IN_DAY = 24 * 60 * 60 * 1000;
const ILLEGAL_COOKIE_PATTERN = /^[^=]+./;

/** Cookie interface. */
export class Cookie {
    private name: string;
    private value: string;
    private days_duration: number;
    private expiration_date: Date | undefined;
    public on_change: ((value: string | number) => void) | undefined;

    /**
     * Constructor.
     *
     * @param name - Cookie identifier.
     * @param value - Value of cookie. If not set will use Cookie.read to find
     * the existing value of the cookie with name equal to the passed name. If
     * the cookie is not found value will be an empty string.
     * @param days_duration - Lifespan in days. If set to less than or equal to
     * 0 will set the lifespan to that of the session.
     */
    constructor(
        name: string,
        value?: string | number | undefined,
        days_duration = 0,
    ) {
        this.name = name;
        this.days_duration = days_duration;
        this.expiration_date = this.initialize_expiration(days_duration);

        // If value is set then simply assign it to this.value. If no value was
        // provided load it from the cookie string. If all else fails simply set
        // the value to an empty string.
        if (value) {
            this.value = this.val(value);
        } else {
            this.value = this.val(this.initialize_empty_value(name));
        }
    }

    /**
     * Constructor helper. Initializes the cookie lifespan based on a number of
     * days to live.
     *
     * @param days_duration - Lifespan in days. If set to less than or equal to
     * 0 will set the lifespan to that of the session.
     * @returns Date of cookie expiration.
     */
    private initialize_expiration(days_duration: number): Date | undefined {
        if (days_duration > 0) {
            const expiration_date = new Date();
            expiration_date.setTime(
                expiration_date.getTime() + days_duration * 24 * 60 * 60 * 1000,
            );
            return expiration_date;
        }
        return;
    }

    /**
     * Constructor helper. Initializes the value of the cookie.
     *
     * @param name - Cookie identifier.
     * @returns The value to set the cookie to.
     */
    private initialize_empty_value(name: string): string {
        const cookie_regex = new RegExp(`${name}[^;]+`);
        const existing_cookie_string = cookie_regex.exec(document.cookie);

        if (existing_cookie_string) {
            const cleaned_cookie_string = existing_cookie_string
                .toString()
                .replace(ILLEGAL_COOKIE_PATTERN, "");
            return decodeURIComponent(cleaned_cookie_string);
        }
        return "";
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
            this.value = `${value}`;
            this.attach();
            if (typeof this.on_change === "function") {
                this.on_change(value);
            }
        }
        return this.value;
    }

    /**
     * Attatches the cookie to the document.
     *
     * @returns - This object for chaining functions.
     */
    public attach(): Cookie {
        let expiration = "";

        if (this.expiration_date) {
            expiration = `; expires=${this.expiration_date.toUTCString()}`;
        }

        const cookie_key_value = `${this.name}=${this.value || ""}`;
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
        document.cookie = `${this.name}=; Max-Age=-99999999`;
        return this;
    }

    /**
     * Updates the expiration date of the cookie and sets it equal to the
     * current date plus the pre-defined lifespan in days.
     *
     * @returns This object for chaining functions.
     */
    public refresh(): Cookie {
        if (this.days_duration > 0) {
            const exp_date = new Date();
            const exp_time = exp_date.getTime();
            exp_date.setTime(exp_time + this.days_duration * MS_IN_DAY);
            this.expiration_date = exp_date;
        } else {
            this.expiration_date = undefined;
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
        return this.value;
    }
}
