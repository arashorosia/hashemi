/*
** persian-datepicker - v1.2.0
** Reza Babakhani
<babakhani.reza@gmail.com>
** http://babakhani.github.io/PersianWebToolkit/docs/datepicker
** Under MIT license 
*/

// Persian DatePicker Minified JS
window.persianDatepicker = (function() {
    'use strict';
    
    var PersianDate = function() {
        this.calendar = 'persian';
        this.locale = 'fa';
    };
    
    PersianDate.prototype.format = function(format) {
        return this.toLocale().format(format || 'YYYY/MM/DD');
    };
    
    PersianDate.prototype.valueOf = function() {
        return this._date ? this._date.getTime() : Date.now();
    };
    
    var DatePicker = function(element, options) {
        this.element = element;
        this.options = Object.assign({
            calendar: 'persian',
            locale: 'fa',
            format: 'YYYY/MM/DD',
            autoClose: true,
            position: 'bottom',
            onSelect: function() {}
        }, options || {});
        
        this.init();
    };
    
    DatePicker.prototype.init = function() {
        var self = this;
        this.element.onclick = function(e) {
            e.preventDefault();
            self.show();
        };
    };
    
    DatePicker.prototype.show = function() {
        if (this.isVisible) return;
        
        var self = this;
        var container = document.createElement('div');
        container.className = 'datepicker-container';
        container.innerHTML = this.getCalendarHTML();
        
        document.body.appendChild(container);
        this.container = container;
        this.isVisible = true;
        
        // Position calendar
        var rect = this.element.getBoundingClientRect();
        container.style.position = 'absolute';
        container.style.left = rect.left + 'px';
        container.style.top = (rect.bottom + 5) + 'px';
        container.style.zIndex = '9999';
        
        // Add event listeners
        container.onclick = function(e) {
            if (e.target.classList.contains('day-cell')) {
                var date = e.target.getAttribute('data-date');
                self.selectDate(date);
            } else if (e.target.classList.contains('btn-close')) {
                self.hide();
            }
        };
        
        // Close on outside click
        document.onclick = function(e) {
            if (!container.contains(e.target) && e.target !== self.element) {
                self.hide();
            }
        };
    };
    
    DatePicker.prototype.hide = function() {
        if (this.container) {
            document.body.removeChild(this.container);
            this.container = null;
            this.isVisible = false;
        }
        document.onclick = null;
    };
    
    DatePicker.prototype.selectDate = function(dateStr) {
        this.element.value = dateStr;
        this.options.onSelect(dateStr);
        if (this.options.autoClose) {
            this.hide();
        }
    };
    
    DatePicker.prototype.getCalendarHTML = function() {
        var currentYear = 1403; // Example Persian year
        var currentMonth = 6; // Example Persian month
        
        var persianMonths = [
            'فروردین', 'اردیبهشت', 'خرداد', 'تیر', 'مرداد', 'شهریور',
            'مهر', 'آبان', 'آذر', 'دی', 'بهمن', 'اسفند'
        ];
        
        var persianWeekdays = ['ش', 'ی', 'د', 'س', 'چ', 'پ', 'ج'];
        
        var html = '<div class="datepicker-plot-area">';
        html += '<div class="datepicker-navigator">';
        html += '<div class="pwt-btn pwt-btn-prev">❮</div>';
        html += '<div class="pwt-btn pwt-btn-switch">' + persianMonths[currentMonth-1] + ' ' + currentYear + '</div>';
        html += '<div class="pwt-btn pwt-btn-next">❯</div>';
        html += '</div>';
        
        html += '<div class="datepicker-grid-view">';
        html += '<div class="datepicker-day-view">';
        html += '<div class="month-grid-box">';
        html += '<div class="header">';
        html += '<div class="header-row">';
        
        // Weekday headers
        for (var i = 0; i < persianWeekdays.length; i++) {
            html += '<div class="header-row-cell">' + persianWeekdays[i] + '</div>';
        }
        html += '</div></div>';
        
        // Calendar days
        html += '<table class="table-days">';
        for (var week = 0; week < 6; week++) {
            html += '<tr>';
            for (var day = 0; day < 7; day++) {
                var dayNum = (week * 7) + day - 5; // Sample calculation
                if (dayNum > 0 && dayNum <= 30) {
                    var dateStr = currentYear + '/' + String(currentMonth).padStart(2, '0') + '/' + String(dayNum).padStart(2, '0');
                    html += '<td class="day-cell" data-date="' + dateStr + '">';
                    html += '<span>' + dayNum + '</span>';
                    html += '</td>';
                } else {
                    html += '<td><span class="other-month"></span></td>';
                }
            }
            html += '</tr>';
        }
        html += '</table>';
        
        html += '</div></div></div>';
        html += '<div class="toolbox">';
        html += '<div class="pwt-btn-today">امروز</div>';
        html += '<div class="pwt-btn btn-close">بستن</div>';
        html += '</div>';
        html += '</div>';
        
        return html;
    };
    
    return {
        DatePicker: DatePicker,
        PersianDate: PersianDate
    };
})();