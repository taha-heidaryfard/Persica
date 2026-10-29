import React from 'react';
import Link from 'next/link';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';
import { Play, Sparkles, Terminal } from 'lucide-react';

export default function QuickstartPageFa() {
  const meta = {
    title: 'اولین برنامه و شروع سریع',
    category: 'شروع کار',
    description: 'نوشتن، اجرا و بررسی اولین برنامه با زبان پرسیکا در کمتر از ۲ دقیقه.',
  };

  const toc = [
    { title: 'فلسفه اجرای بدون تشریفات (Zero-Ceremony)', id: 'philosophy', level: 2 },
    { title: 'اسکریپت سلام دنیا (Hello World)', id: 'hello-world', level: 2 },
    { title: 'ورودی و خروجی تعاملی (I/O)', id: 'io', level: 2 },
    { title: 'مثال جامع: تولید دنباله فیبوناچی', id: 'fibonacci', level: 2 },
    { title: 'گام‌های بعدی یادگیری', id: 'next-steps', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="philosophy" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        فلسفه اجرای بدون تشریفات (Zero-Ceremony)
      </h2>
      <p>
        برخلاف زبان‌هایی مانند جاوا یا سی‌پلاس‌پلاس که برای چاپ یک پیام ساده نیازمند تعریف کلاس، تابع main، پکیج‌ها و داربست‌بندی پیچیده هستند، پرسیکا از فلسفه <strong>اجرای مستقیم و بدون تشریفات</strong> پیروی می‌کند:
      </p>
      <p className="text-emerald-300 font-medium">
        هر دستور معتبر پرسیکا در بالاترین سطح فایل (Top-level) بلافاصله و بدون نیاز به تابع main اجرا می‌شود.
      </p>

      <h2 id="hello-world" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        اسکریپت سلام دنیا (Hello World)
      </h2>
      <p>
        یک فایل به نام <code className="text-emerald-300">hello.persi</code> بسازید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="hello.persi"
          code={`// اولین اسکریپت پرسیکای شما
print("Hello, world! 🌿");
print("به نسخه ۰.۱۳.۰ زبان پرسیکا خوش آمدید!");`}
        />
      </div>

      <p>آن را مستقیماً از طریق ترمینال اجرا کنید:</p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`persica hello.persi`}
          output={`Hello, world! 🌿
به نسخه ۰.۱۳.۰ زبان پرسیکا خوش آمدید!`}
        />
      </div>

      <h2 id="io" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ورودی و خروجی تعاملی (I/O)
      </h2>
      <p>
        پرسیکا تابع پایتونی <code className="text-emerald-400">input(prompt)</code> و دستور چند آرگومانی <code className="text-emerald-400">print()</code> را ارائه می‌دهد:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="greeting.persi"
          code={`// دریافت نام کاربر از ورودی
string name = input("نام خود را وارد کنید: ");

// دریافت سن و تبدیل آن به عدد صحیح با to_int
int age = to_int(input("سن خود را وارد کنید: "));

print("درود بر تو", name, "!");
print("پنج سال بعد سن شما برابر خواهد بود با:", age + 5);`}
          output={`نام خود را وارد کنید: علی
سن خود را وارد کنید: 25
درود بر تو علی !
پنج سال بعد سن شما برابر خواهد بود با: 30`}
        />
      </div>

      <h2 id="fibonacci" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        مثال جامع: تولید دنباله فیبوناچی
      </h2>
      <p>
        در این قطعه کد نحوه تعریف تابع، نوع‌دهی ایستا، کار با آرایه‌ها و حلقه را مشاهده می‌کنید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="fibonacci.persi"
          code={`// تابعی برای محاسبه n عدد اول دنباله فیبوناچی
int[] generate_fibonacci(int count) {
    int[] fib = [0, 1];
    
    for (int i = 2; i < count; i++) {
        int next_val = fib[i - 1] + fib[i - 2];
        append(fib, next_val);
    }
    
    return fib;
}

int count = 10;
print("تولید ۱۰ عدد اول دنباله فیبوناچی:");

int[] result = generate_fibonacci(count);

for idx in range(0, len(result)) {
    print("Fib(" + to_string(idx) + ") =", result[idx]);
}`}
          output={`تولید ۱۰ عدد اول دنباله فیبوناچی:
Fib(0) = 0
Fib(1) = 1
Fib(2) = 1
Fib(3) = 2
Fib(4) = 3
Fib(5) = 5
Fib(6) = 8
Fib(7) = 13
Fib(8) = 21
Fib(9) = 34`}
        />
      </div>

      <h2 id="next-steps" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        گام‌های بعدی یادگیری
      </h2>
      <p>
        اکنون که با نحوه اجرای اسکریپت‌ها آشنا شدید، سایر قابلیت‌های زبان را کشف کنید:
      </p>
      <ul className="text-xs text-slate-300 list-disc pr-5 space-y-1.5 my-3">
        <li>آشنایی با ایجاد پروژه‌های چند فایلی در <Link href="/fa/docs/project-system" className="text-emerald-400 font-semibold underline">پیکربندی persica.toml و .persi_env</Link>.</li>
        <li>بررسی انواع داده ایستا و متغیرها در <Link href="/fa/docs/syntax-basics" className="text-emerald-400 font-semibold underline">متغیرها و انواع داده</Link>.</li>
        <li>تسلط بر برنامه‌نویسی شیء‌گرا در <Link href="/fa/docs/oop" className="text-emerald-400 font-semibold underline">استراکت‌ها و کلاس‌ها (OOP)</Link>.</li>
      </ul>
    </DocsLayout>
  );
}
