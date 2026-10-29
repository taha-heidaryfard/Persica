import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';

export default function FunctionsPageFa() {
  const meta = {
    title: 'توابع، امضا و فراخوانی بازگشتی',
    category: 'هسته زبان',
    description: 'تعریف توابع با نوع بازگشتی ایستا، پارامترهای تایپ‌شده، اعتبارسنجی مسیرهای return، توابع void و الگوریتم‌های بازگشتی.',
  };

  const toc = [
    { title: 'تعریف توابع تایپ‌شده', id: 'declaring', level: 2 },
    { title: 'توابع بدون خروجی (void)', id: 'void-fns', level: 2 },
    { title: 'بررسی صحت مسیرهای بازگشت (Return Path Safety)', id: 'return-paths', level: 2 },
    { title: 'بازگشت و فراخوانی متقابل (Recursion)', id: 'recursion', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="declaring" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        تعریف توابع تایپ‌شده
      </h2>
      <p>
        در پرسیکا تمام توابع دارای نوع بازگشتی و پارامترهای مشخص هستند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="functions.persi"
          code={`// تابعی با دو ورودی صحیح و خروجی صحیح
int multiply(int a, int b) {
    return a * b;
}

// تابعی با ورودی اعشاری برای محاسبه مساحت دایره
float circle_area(float radius) {
    final float PI = 3.14159265;
    return PI * radius * radius;
}

print("5 * 8 =", multiply(5, 8));
print("Area of radius 4.0 =", circle_area(4.0));`}
          output={`5 * 8 = 40
Area of radius 4.0 = 50.2654824`}
        />
      </div>

      <h2 id="void-fns" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        توابع بدون خروجی (void)
      </h2>
      <p>
        توابعی که صرفاً عملیاتی را انجام می‌دهند و مقداری بازنمی‌گردانند، با نوع <code className="text-emerald-400">void</code> تعریف می‌شوند. برای خروج زودهنگام می‌توانید از <code className="text-emerald-400">return;</code> به تنهایی استفاده کنید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="void_functions.persi"
          code={`void log_warning(string message, bool is_critical) {
    if !is_critical {
        print("[INFO]", message);
        return; // خروج زودهنگام
    }
    print("[CRITICAL ALERT]", message);
}

log_warning("Cache refreshed", false);
log_warning("Database connection timeout", true);`}
          output={`[INFO] Cache refreshed
[CRITICAL ALERT] Database connection timeout`}
        />
      </div>

      <h2 id="return-paths" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        بررسی صحت مسیرهای بازگشت (Return Path Safety)
      </h2>
      <p>
        تحلیل‌گر معنایی پرسیکا بررسی می‌کند که <strong>تمامی شاخه‌های ممکن</strong> یک تابع با خروجی غیر void، حتماً مقدار بازگردانند. در غیر این صورت خطای کامپایل صادر می‌شود:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="return_safety.persi"
          code={`// معتبر: هر دو مسیر if و else مقدار int برمی‌گردانند
int absolute_value(int n) {
    if n < 0 {
        return -n;
    } else {
        return n;
    }
}

// معتبر: دستور throw نیز شرط بازگشت مسیر را برآورده می‌کند
string get_status(int code) {
    if code == 200 {
        return "OK";
    }
    throw "کد وضعیت نامعتبر است";
}`}
        />
      </div>

      <h2 id="recursion" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        بازگشت و فراخوانی متقابل (Recursion)
      </h2>
      <p>
        پرسیکا بدون نیاز به پیش‌تعریف (Forward Declaration)، فراخوانی بازگشتی و فراخوانی متقابل بین توابع را پشتیبانی می‌کند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="recursion.persi"
          code={`// تابع بازگشتی فاکتوریل
int factorial(int n) {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

// بازگشت متقابل: is_even تابع is_odd را صدا می‌زند که پایین‌تر تعریف شده
bool is_even(int n) {
    if n == 0 { return true; }
    return is_odd(n - 1);
}

bool is_odd(int n) {
    if n == 0 { return false; }
    return is_even(n - 1);
}

print("5! =", factorial(5));
print("Is 14 even?", is_even(14));`}
          output={`5! = 120
Is 14 even? true`}
        />
      </div>
    </DocsLayout>
  );
}
