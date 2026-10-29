import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';
import { Sparkles, Download, CheckCircle, Code2, Terminal } from 'lucide-react';

export default function VSCodePageFa() {
  const meta = {
    title: 'افزونه رسمی وی‌اس‌کد (VS Code)',
    category: 'ابزارها و اکوسیستم',
    badge: 'نسخه رسمی ۰.۱۳',
    description: 'تبدیل محیط ویژوال استودیو کد به یک IDE کامل و هوشمند برای زبان برنامه‌نویسی پرسیکا.',
  };

  const toc = [
    { title: 'امکانات و قابلیت‌های افزونه', id: 'overview', level: 2 },
    { title: 'نصب فایل پکیج (.vsix)', id: 'installation', level: 2 },
    { title: 'پیشنهاد هوشمند کد (IntelliSense)', id: 'intellisense', level: 2 },
    { title: 'راهنمای هاور (Hover Documentation)', id: 'hover', level: 2 },
    { title: 'تشخیص زنده خطاها هنگام ذخیره', id: 'diagnostics', level: 2 },
    { title: 'دکمه‌های ۱-کلیک اجرا و تست', id: 'commands', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="overview" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        امکانات و قابلیت‌های افزونه
      </h2>
      <p>
        افزونه رسمی پرسیکا در VS Code تجربه توسعه روان، مدرن و هماهنگ با نیازهای برنامه‌نویسان پرسیکا فراهم می‌آورد:
      </p>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 my-6 text-xs">
        <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60">
          <div className="font-bold text-emerald-400 text-sm mb-1">هوشمندی محیط ادیتور</div>
          <p className="text-slate-400">هایلایت سینتکس دقیق برای تمامی کلمات کلیدی، انواع داده، نمادهای پیش‌درآمد، راهنمای هاور و تکمیل خودکار کدها.</p>
        </div>
        <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60">
          <div className="font-bold text-emerald-400 text-sm mb-1">تشخیص خطای آنی</div>
          <p className="text-slate-400">بررسی پس‌زمینه پروژه هنگام ذخیره فایل و نمایش خطاهای نگارشی و مغایرت‌های نوع داده با خط قرمز زیر کلمات.</p>
        </div>
      </div>

      <h2 id="installation" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        نصب فایل پکیج (`.vsix`)
      </h2>
      <p>
        این افزونه به صورت یک بسته مستقل <code className="text-emerald-400">.vsix</code> آماده دانلود و نصب است:
      </p>

      <div className="my-4">
        <a
          href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/persica-0.13.0.vsix"
          className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-emerald-500 hover:bg-emerald-600 text-white font-bold text-xs shadow-lg shadow-emerald-500/30 transition transform hover:-translate-y-0.5"
        >
          <Download className="w-4 h-4" />
          <span>دانلود فایل persica-0.13.0.vsix</span>
        </a>
      </div>

      <p className="text-xs text-slate-400">نصب مستقیم از طریق خط فرمان:</p>
      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`code --install-extension persica-0.13.0.vsix`}
        />
      </div>

      <p className="text-xs text-slate-400 mt-2">
        یا در محیط VS Code: باز کردن زبانه <strong>Extensions</strong> با کلیدهای (<kbd className="bg-slate-800 px-1.5 py-0.5 rounded border border-slate-700">Ctrl+Shift+X</kbd>)، کلیک روی منوی سه نقطه (<code className="text-slate-200">···</code>) و انتخاب گزینه <strong>"Install from VSIX..."</strong>.
      </p>

      <h2 id="intellisense" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        پیشنهاد هوشمند کد (IntelliSense)
      </h2>
      <p>
        هنگام تایپ در فایل‌های با پسوند <code className="text-emerald-300">.persi</code> پیشنهادات هوشمند ظاهر می‌شوند:
      </p>
      <ul className="text-xs text-slate-300 list-disc pr-5 space-y-1 my-3">
        <li>تمامی توابع توکار با امضا و پارامترها (<code className="text-emerald-400">print</code>, <code className="text-emerald-400">input</code>, <code className="text-emerald-400">len</code>, <code className="text-emerald-400">range</code>, <code className="text-emerald-400">append</code>, ...).</li>
        <li>کلاس‌های استاندارد پیش‌درآمد (<code className="text-emerald-400">Stack</code>, <code className="text-emerald-400">Queue</code>, <code className="text-emerald-400">Set</code>, <code className="text-emerald-400">StringBuilder</code>, <code className="text-emerald-400">Result</code>, <code className="text-emerald-400">Option</code>).</li>
        <li>انواع داده‌های پایه و مجموعه‌ها (<code className="text-emerald-400">int</code>, <code className="text-emerald-400">float</code>, <code className="text-emerald-400">string</code>, <code className="text-emerald-400">bool</code>, <code className="text-emerald-400">void</code>, <code className="text-emerald-400">map</code>, <code className="text-emerald-400">dict</code>).</li>
        <li>کلمات کلیدی کنترلی، حلقه‌ها و عبارات تعریفی.</li>
      </ul>

      <h2 id="hover" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        راهنمای هاور (Hover Documentation)
      </h2>
      <p>
        با بردن نشانگر ماوس روی هر تابع، کلمه کلیدی یا کلاس، پنجره راهنمای فرمت‌شده شامل امضا، انواع آرگومان‌ها، رفتار و توضیحات تفصیلی به نمایش درمی‌آید.
      </p>

      <h2 id="diagnostics" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        تشخیص زنده خطاها هنگام ذخیره
      </h2>
      <p>
        با هر بار ذخیره فایل <code className="text-emerald-300">.persi</code>، افزونه در پس‌زمینه دستور <code className="text-emerald-300">persica --check</code> را فراخوانی می‌کند. در صورت وجود هرگونه خطای سینتکسی یا ناسازگاری در انواع داده‌ها، خطا بلافاصله زیر خط مربوطه با موج قرمز مشخص شده و در پنل مشکلات (<strong>Problems Panel</strong> با کلیدهای <kbd className="bg-slate-800 px-1.5 py-0.5 rounded border border-slate-700">Ctrl+Shift+M</kbd>) فهرست می‌گردد.
      </p>

      <h2 id="commands" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        دکمه‌های ۱-کلیک اجرا و تست
      </h2>
      <p>
        افزونه دکمه‌های سریعی را مستقیماً در نوار بالای ادیتور اضافه می‌کند:
      </p>
      <ul className="text-xs text-slate-300 list-disc pr-5 space-y-1 my-3">
        <li><strong>دکمه اجرا (Play):</strong> فایل فعال را مستقیماً در ترمینال یکپارچه پرسیکا اجرا می‌کند.</li>
        <li><strong>دکمه بررسی (Check):</strong> بررسی صحت انواع و نگارش فایل جاری بدون اجرای آن.</li>
      </ul>

      <p className="text-xs text-slate-400 mt-2">فرمان‌های موجود در پالت دستورات (<kbd className="bg-slate-800 px-1.5 py-0.5 rounded border border-slate-700">Ctrl+Shift+P</kbd>):</p>
      <div className="p-3 rounded-xl border border-slate-800 bg-slate-900/60 font-mono text-xs text-slate-300 my-2 space-y-1" dir="ltr">
        <div>&gt; Persica: Run Current File</div>
        <div>&gt; Persica: Check Syntax & Types</div>
        <div>&gt; Persica: Run Project (persica run)</div>
        <div>&gt; Persica: Run Tests (persica test)</div>
        <div>&gt; Persica: Create New Project (persica new)</div>
      </div>
    </DocsLayout>
  );
}
