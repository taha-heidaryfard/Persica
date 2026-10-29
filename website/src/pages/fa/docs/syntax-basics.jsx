import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';

export default function SyntaxBasicsPageFa() {
  const meta = {
    title: 'متغیرها و انواع داده‌های ایستا',
    category: 'هسته زبان',
    description: 'آشنایی با سیستم نوع‌دهی ایستا در پرسیکا، انواع داده پایه، تبدیل صریح نوع (Casting) و متغیرهای تغییرناپذیر final.',
  };

  const toc = [
    { title: 'انواع داده پایه (Primitives)', id: 'primitives', level: 2 },
    { title: 'تعریف متغیرها', id: 'declarations', level: 2 },
    { title: 'متغیرهای تغییرناپذیر با final', id: 'final', level: 2 },
    { title: 'تبدیل صریح نوع (Type Casting)', id: 'conversions', level: 2 },
    { title: 'عملگرهای حسابی و منطقی', id: 'operators', level: 2 },
    { title: 'حوزه متغیرها و Shadowing', id: 'shadowing', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="primitives" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        انواع داده پایه (Primitive Types)
      </h2>
      <p>
        پرسیکا دارای یک سیستم نوع‌دهی ایستا (Static & Strong) است که پیش از اجرای برنامه، تمامی انواع را بررسی و تضمین می‌کند:
      </p>

      <div className="overflow-x-auto my-4">
        <table className="w-full text-right text-xs border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">نوع (Type)</th>
              <th className="p-3">توضیحات</th>
              <th className="p-3">نمونه مقادیر</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">int</td>
              <td className="p-3">عدد صحیح ۶۴ بیتی علامت‌دار</td>
              <td className="p-3 font-mono" dir="ltr">0, 42, -100</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">float</td>
              <td className="p-3">عدد اعشاری ۶۴ بیتی ممیز شناور (IEEE 754)</td>
              <td className="p-3 font-mono" dir="ltr">3.14159, -0.05, 1e6</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">string</td>
              <td className="p-3">رشته کاراکتری یونیکد UTF-8 با کاراکترهای اسکیپ</td>
              <td className="p-3 font-mono" dir="ltr">"درود", "Persica 🌿", "\n"</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">bool</td>
              <td className="p-3">مقدار منطقی بولی</td>
              <td className="p-3 font-mono" dir="ltr">true, false</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">void</td>
              <td className="p-3">عدم بازگشت مقدار (برای توابعی که خروجی ندارند)</td>
              <td className="p-3 font-mono" dir="ltr">-</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">any</td>
              <td className="p-3">نوع داینامیک برای نگهداری مقادیر ناهمگن در صورت نیاز</td>
              <td className="p-3 font-mono" dir="ltr">-</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2 id="declarations" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        تعریف متغیرها
      </h2>
      <p>
        برای تعریف متغیر ابتدا نوع داده، سپس نام متغیر و در انتها مقدار اولیه آن را مشخص می‌کنیم:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="variables.persi"
          code={`int count = 10;
float pi = 3.14159265;
string language = "Persica";
bool is_production = true;

// متغیرهای معمولی می‌توانند مقدار جدید بگیرند
count = 20;
print("Current count:", count);`}
          output={`Current count: 20`}
        />
      </div>

      <h2 id="final" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        متغیرهای تغییرناپذیر با `final`
      </h2>
      <p>
        برای جلوگیری از تغییر تصادفی مقدار متغیر، کلمه کلیدی <code className="text-emerald-400 font-bold">final</code> را قبل از نوع متغیر قرار دهید. کامپایلر در زمان استاتیک خطای تغییر مقدار را گزارش می‌دهد:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="immutable.persi"
          code={`final string API_URL = "https://api.persica-lang.org/v1";
final int MAX_RETRIES = 5;

// تلاش برای انتساب مقدار جدید خطای کامپایل می‌دهد:
// API_URL = "https://other.org"; // Error: cannot reassign to final variable 'API_URL'

print("API configured at:", API_URL);`}
          output={`API configured at: https://api.persica-lang.org/v1`}
        />
      </div>

      <h2 id="conversions" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        تبدیل صریح نوع (Explicit Type Conversion)
      </h2>
      <p>
        پرسیکا برای جلوگیری از خطاهای پنهان، اجازه تبدیل ضمنی و خودکار بین انواع ناسازگار را نمی‌دهد. تمام تبدیلات باید صریح انجام شوند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="casting.persi"
          code={`// تبدیل رشته به عدد صحیح یا اعشاری
int parsed_int = to_int("450");
float parsed_float = to_float("12.75");

// تبدیل عدد صحیح به اعشاری و برعکس
float ratio = to_float(450) / 100.0;
int rounded = to_int(12.75); // بخش اعشار حذف می‌شود

// تبدیل هر مقداری به رشته
string text = to_string(parsed_int) + " آیتم";
print(text, "با نسبت", ratio);`}
          output={`450 آیتم با نسبت 4.5`}
        />
      </div>

      <h2 id="operators" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        عملگرهای حسابی و منطقی
      </h2>
      <p>
        پرسیکا از تمامی عملگرهای استاندارد با قابلیت Short-circuiting کامل پشتیبانی می‌کند:
      </p>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 my-4 text-xs">
        <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60">
          <h4 className="font-bold text-emerald-400 mb-2">عملگرهای حسابی:</h4>
          <ul className="space-y-1 text-slate-300 font-mono" dir="ltr">
            <li>+ &nbsp;- &nbsp;* &nbsp;/ &nbsp;%</li>
            <li>+= &nbsp;-= &nbsp;*= &nbsp;/= &nbsp;%=</li>
            <li>++ &nbsp;-- &nbsp;(پیشوند و پسوند)</li>
          </ul>
        </div>
        <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60">
          <h4 className="font-bold text-emerald-400 mb-2">عملگرهای مقایسه‌ای و منطقی:</h4>
          <ul className="space-y-1 text-slate-300 font-mono" dir="ltr">
            <li>== &nbsp;!= &nbsp;&lt; &nbsp;&gt; &nbsp;&lt;= &nbsp;&gt;=</li>
            <li>&& &nbsp;|| &nbsp;! &nbsp;(اتصال کوتاه)</li>
            <li>and, or, not (پشتیبانی کلیدواژه‌ای پایتونی)</li>
          </ul>
        </div>
      </div>

      <h2 id="shadowing" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        حوزه متغیرها و پنهان‌سازی (Variable Shadowing)
      </h2>
      <p>
        پرسیکا دارای سیستم حوزه بلوکی صلب (Lexical Scoping) است. بلاک‌های درونی می‌توانند بدون دستکاری متغیر بیرونی، متغیری همنام تعریف کنند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="scoping.persi"
          code={`int value = 100;

{
    // بلوک درونی متغیر بیرونی را سایه می‌کند
    int value = 999;
    print("Inside block:", value);
}

print("Outside block:", value);`}
          output={`Inside block: 999
Outside block: 100`}
        />
      </div>
    </DocsLayout>
  );
}
