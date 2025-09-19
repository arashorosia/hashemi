// کتابخانه تبدیل تاریخ شمسی به میلادی - نسخه ساده و دقیق
class PersianDate {
    constructor() {
        this.persianMonths = [
            'فروردین', 'اردیبهشت', 'خرداد', 'تیر', 'مرداد', 'شهریور',
            'مهر', 'آبان', 'آذر', 'دی', 'بهمن', 'اسفند'
        ];
    }

    // تبدیل تاریخ شمسی به میلادی با فرمول دقیق جوليان
    shamsiToMiladi(jy, jm, jd) {
        let gy, gm, gd;
        jy += 1595;
        let days = -355668 + (365 * jy) + (Math.floor(jy / 33) * 8) + Math.floor(((jy % 33) + 3) / 4) + jd;
        if (jm < 7) {
            days += (jm - 1) * 31;
        } else {
            days += ((jm - 7) * 30) + 186;
        }

        gy = 400 * Math.floor(days / 146097);
        days %= 146097;

        let leap = true;
        if (days >= 36525) {
            days--;
            gy += 100 * Math.floor(days / 36524);
            days %= 36524;
            if (days >= 365) {
                days++;
            }
            leap = false;
        }

        gy += 4 * Math.floor(days / 1461);
        days %= 1461;

        if (days >= 366) {
            leap = false;
            days--;
            gy += Math.floor(days / 365);
            days = days % 365;
        }

        gd = days + 1;

        const sal_a = [0, 31, (leap ? 29 : 28), 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        
        for (gm = 0; gm < 13; gm++) {
            let v = sal_a[gm];
            if (gd <= v) break;
            gd -= v;
        }

        return { year: gy, month: gm, day: gd };
    }

    // تبدیل تاریخ میلادی به شمسی
    miladiToShamsi(gy, gm, gd) {
        const g_d_m = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        
        let jy = (gy <= 1600) ? 0 : 979;
        gy -= (gy <= 1600) ? 621 : 1600;
        
        let gy2 = (gm > 2) ? (gy + 1) : gy;
        let days = (365 * gy) + (Math.floor((gy2 + 3) / 4)) - (Math.floor((gy2 + 99) / 100)) + 
                   (Math.floor((gy2 + 399) / 400)) - 80 + gd + g_d_m[gm - 1];
        
        jy += 33 * Math.floor(days / 12053);
        days %= 12053;
        
        jy += 4 * Math.floor(days / 1461);
        days %= 1461;
        
        if (days > 365) {
            jy += Math.floor((days - 1) / 365);
            days = (days - 1) % 365;
        }
        
        let jm, jd;
        if (days < 186) {
            jm = 1 + Math.floor(days / 31);
            jd = 1 + (days % 31);
        } else {
            jm = 7 + Math.floor((days - 186) / 30);
            jd = 1 + ((days - 186) % 30);
        }
        
        return { year: jy, month: jm, day: jd };
    }

    // فرمت کردن تاریخ شمسی
    formatPersianDate(year, month, day) {
        return `${year}/${String(month).padStart(2, '0')}/${String(day).padStart(2, '0')}`;
    }

    // فرمت کردن تاریخ میلادی
    formatGregorianDate(year, month, day) {
        return `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
    }
}

// تست مستقل
console.log("=== تست کتابخانه جدید ===");
const pd = new PersianDate();

// تست 24 آذر 1366
console.log("تست: 24 آذر 1366");
const result1 = pd.shamsiToMiladi(1366, 9, 24);
console.log("نتیجه:", result1);
console.log("انتظار: 15 دسامبر 1987");

// تست معکوس
console.log("\nتست معکوس: 15 دسامبر 1987");
const result2 = pd.miladiToShamsi(1987, 12, 15);
console.log("نتیجه:", result2);
console.log("انتظار: 24 آذر 1366");