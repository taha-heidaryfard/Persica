# ماژول‌ها و سیستم ایمپورت در پرسیان (Persica Modules & Imports)

در نسخه **0.10.0** زبان پرسیان، سیستم ماژولار کامل به سبک پایتون (Python-style Modularity) پیاده‌سازی شده است. تمام فایل‌های `.persi` به عنوان ماژول قابل استفاده هستند و برای دسترسی و استفاده از توابع، داده‌مدل‌ها (`struct`)، کلاس‌ها (`class`) و متغیرها نیازی به کوتیشن یا پسوند فایل نیست و همه چیز به صورت عمومی (Public by default) صادر می‌شود.

---

## ۱. ساختار ماژول‌ها

هر فایل با پسوند `.persi` یک ماژول مستقل است. نام فایل یا ساختار پوشه‌ای آن مشخص‌کننده مسیر ماژول است:

```text
my_project/
├── math_utils.persi
├── models/
│   └── geometry.persi
└── main.persi
```

- نیازی به نوشتن `.persi` یا استفاده از `""` نیست.
- زیرپوشه‌ها با علامت نقطه (`.`) جدا می‌شوند (مانند `models.geometry`).

---

## ۲. انواع ساختار ایمپورت (Import Styles)

### الف) ایمپورت کل ماژول (`import module;`)
ماژول به صورت یک شیء / نیم‌اسپیس وارد حوزه دید جاری می‌شود و اعضای آن با نقطه فراخوانی می‌شوند:

```persica
import math_utils;

print(math_utils.PI);
print(math_utils.add(10, 20));
```

### ب) ایمپورت با نام مستعار (`import module as alias;`)
برای کوتاه‌تر کردن نام یا جلوگیری از تداخل:

```persica
import math_utils as m;

print(m.factorial(5));
```

### ج) ایمپورت مستقیم نمادها (`from module import symbol1, symbol2;`)
نمادهای مورد نظر (توابع، استراکت‌ها، کلاس‌ها، ثابت‌ها) مستقیماً وارد حوزه دید فایل فعلی می‌شوند:

```persica
from models.geometry import Point, Circle;

Point p = Point(10, 20);
```

### د) گروه‌بندی با پرانتز یا آکولاد (`()`, `{}`)
پرسیان برای ایمپورت‌های چند خطی یا مرتب، علاوه بر لیست کاما‌جدا از پرانتز `()` و آکولاد `{}` نیز پشتیبانی می‌کند:

```persica
from models.geometry import (
    Point,
    Circle,
    distance
);

from models.geometry import { Point, Circle };
```

### ه) ایمپورت نماد با نام مستعار (`from module import Item as Alias;`)
```persica
from models.geometry import Point as Pt, make_point as mk_pt;

Pt p = mk_pt(5, 7);
print(p.x);
```

### و) ایمپورت همگانی (`from module import *;`)
وارد کردن تمام توابع، مدل‌ها و کلاس‌های یک ماژول به صورت یکجا:

```persica
from toolkit import *;
```

---

## ۳. ویژگی‌های کلیدی سیستم ماژول در پرسیان

1. **Public by default:** همه توابع، استراکت‌ها، کلاس‌ها و متغیرهای تعریف‌شده در ماژول به صورت عمومی در دسترس هستند.
2. **Circular Import Detection:** وجود ایمپورت حلقوی (A -> B -> A) به صورت استاتیک شناسایی شده و مانع از سرریز حافظه یا خطای زمان اجرا می‌شود.
3. **Module Caching:** هر ماژول تنها یک‌بار بارگذاری و ارزیابی شده و خروجی آن کش می‌شود.
4. **Strict Type Safety:** تحلیل‌گر معنایی (Semantic Analyzer) انواع داده، امضای توابع و سازنده‌های مدل‌ها را میان ماژول‌ها به صورت کامل اعتبارسنجی می‌کند.
