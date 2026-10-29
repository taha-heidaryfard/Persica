import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';

export default function ControlFlowPageFa() {
  const meta = {
    title: 'شروط و ساختارهای کنترلی',
    category: 'هسته زبان',
    description: 'راهنمای کامل شاخه‌بندی شرطی if/else، حلقه‌های پایتونی for-in، تابع مولد range، حلقه‌های while و دستورات break و continue.',
  };

  const toc = [
    { title: 'شروط if و else', id: 'if-else', level: 2 },
    { title: 'حلقه‌های پایتونی for ... in', id: 'for-in', level: 2 },
    { title: 'تابع مولد range()', id: 'range', level: 2 },
    { title: 'حلقه‌های while', id: 'while', level: 2 },
    { title: 'حلقه‌های سبک سی (C-style for)', id: 'c-for', level: 2 },
    { title: 'دستورات break و continue', id: 'break-continue', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="if-else" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        شروط `if` و `else`
      </h2>
      <p>
        در پرسیکا، گذاشتن پرانتز دور شرط‌ها اختیاری است. می‌توانید ساختار تمیز و خوانای پایتونی یا فرم پرانتزدار متداول در زبان‌های C را به کار ببرید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="conditions.persi"
          code={`int score = 88;

if score >= 90 {
    print("Grade: A");
} else if score >= 80 {
    print("Grade: B");
} else if score >= 70 {
    print("Grade: C");
} else {
    print("Grade: F");
}`}
          output={`Grade: B`}
        />
      </div>

      <h2 id="for-in" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        حلقه‌های پایتونی `for ... in`
      </h2>
      <p>
        پرسیکا پیمایش مستقیم و صریح روی آرایه‌ها و رشته‌ها را با ساختار <code className="text-emerald-400">for item in collection</code> فراهم می‌کند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="for_in.persi"
          code={`// پیمایش مستقیم عناصر یک آرایه
string[] fruits = ["Apple", "Orange", "Pomegranate"];

for fruit in fruits {
    print("Fruit:", fruit);
}

// پیمایش کاراکتر به کاراکتر رشته یونیکد UTF-8
string greeting = "سلام";
for char in greeting {
    print("Char:", char);
}`}
          output={`Fruit: Apple
Fruit: Orange
Fruit: Pomegranate
Char: س
Char: ل
Char: ا
Char: م`}
        />
      </div>

      <h2 id="range" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        تابع مولد `range()`
      </h2>
      <p>
        تابع توکار <code className="text-emerald-400">range()</code> دنباله‌ای از اعداد صحیح را تولید می‌کند و می‌تواند ۱، ۲ یا ۳ آرگومان دریافت نماید:
      </p>

      <div className="overflow-x-auto my-4 text-xs">
        <table className="w-full text-right border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">نحوه فراخوانی</th>
              <th className="p-3">مقادیر تولیدی</th>
              <th className="p-3">توضیحات</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">range(5)</td>
              <td className="p-3 font-mono" dir="ltr">[0, 1, 2, 3, 4]</td>
              <td className="p-3">از صفر تا قبل از مقدار پایانی (نامشمول).</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">range(1, 6)</td>
              <td className="p-3 font-mono" dir="ltr">[1, 2, 3, 4, 5]</td>
              <td className="p-3">از مقدار آغازین تا قبل از پایان.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">range(0, 10, 2)</td>
              <td className="p-3 font-mono" dir="ltr">[0, 2, 4, 6, 8]</td>
              <td className="p-3">با گام حرکت سفارشی (Step).</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">range(5, 0, -1)</td>
              <td className="p-3 font-mono" dir="ltr">[5, 4, 3, 2, 1]</td>
              <td className="p-3">شمارش معکوس با گام منفی.</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="range_demo.persi"
          code={`// شمارش معکوس با گام منفی ۱
print("Countdown starting:");
for i in range(3, 0, -1) {
    print(i, end="... ");
}
print("Liftoff! 🚀");`}
          output={`Countdown starting:
3... 2... 1... Liftoff! 🚀`}
        />
      </div>

      <h2 id="while" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        حلقه‌های `while`
      </h2>
      <p>
        تا زمانی که شرط منطقی حلقه برقرار باشد، بدنه حلقه مرتباً تکرار می‌شود:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="while_demo.persi"
          code={`int n = 1;
while n < 32 {
    print("Power of two:", n);
    n *= 2;
}`}
          output={`Power of two: 1
Power of two: 2
Power of two: 4
Power of two: 8
Power of two: 16`}
        />
      </div>

      <h2 id="c-for" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        حلقه‌های سبک سی (C-Style Indexed `for`)
      </h2>
      <p>
        برای تکرارهای سنتی دارای متغیر شمارنده، شرط توقف و گام افزایش صریح:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="c_style.persi"
          code={`for (int i = 0; i < 3; i++) {
    print("Index:", i);
}`}
          output={`Index: 0
Index: 1
Index: 2`}
        />
      </div>

      <h2 id="break-continue" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        دستورات `break` و `continue`
      </h2>
      <p>
        با <code className="text-emerald-400">break</code> می‌توان بلافاصله از حلقه خارج شد و با <code className="text-emerald-400">continue</code> تکرار فعلی متوقف و گام بعدی آغاز می‌گردد:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="jump.persi"
          code={`for i in range(1, 10) {
    if i % 2 == 0 {
        continue; // رد کردن اعداد زوج
    }
    if i > 7 {
        break; // پایان حلقه در صورت بزرگتر شدن از ۷
    }
    print("Odd:", i);
}`}
          output={`Odd: 1
Odd: 3
Odd: 5
Odd: 7`}
        />
      </div>
    </DocsLayout>
  );
}
