
// اضافه کردن تست console.log برای دیباگ
console.log('تست تبدیل تاریخ:');
const pd = new PersianDate();
console.log('1403/06/29 شمسی برابر است با:', pd.shamsiToMiladi(1403, 6, 29));
console.log('2024/09/19 میلادی برابر است با:', pd.miladiToShamsi(2024, 9, 19));

