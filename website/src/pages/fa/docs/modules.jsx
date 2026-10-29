import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';

export default function ModulesPageFa() {
  const meta = {
    title: 'سیستم پکیج و ایمپورت ماژول‌ها',
    category: 'ماژول‌ها',
    description: 'ساختاربندی پروژه‌های چندفایلی با سیستم ایمپورت پایتونی، نام‌های مستعار با as، پکیج‌های فولدری mod.persi و محافظت در برابر ایمپورت‌های حلقوی.',
  };

  const toc = [
    { title: 'فلسفه سیستم ایمپورت', id: 'philosophy', level: 2 },
    { title: 'روش‌ها و سینتکس ایمپورت', id: 'syntax', level: 2 },
    { title: 'ماژول‌های فولدری (mod.persi)', id: 'folders', level: 2 },
    { title: 'زیرماژول‌ها و ساختار نقطه‌ای', id: 'submodules', level: 2 },
    { title: 'محافظت در برابر ایمپورت‌های دایره‌ای', id: 'circular', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="philosophy" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        فلسفه سیستم ایمپورت
      </h2>
      <p>
        پرسیکا سینتکس روان و خوانای زبان پایتون را برای مدیریت ماژول‌ها برگزیده است:
      </p>
      <ul className="text-xs text-slate-300 list-disc pr-5 space-y-1 my-3">
        <li><strong>بدون پسوند فایل:</strong> بنویسید <code className="text-emerald-400">import math;</code> به جای استفاده از مسیرهای رشته‌ای مانند <code className="text-slate-400">"math.persi"</code>.</li>
        <li><strong>عمومی به صورت پیش‌فرض:</strong> تمام توابع، کلاس‌ها، استراکت‌ها و مقادیر در بالاترین سطح ماژول برای ایمپورت‌کنندگان در دسترس هستند.</li>
        <li><strong>کش‌گذاری در زمان کامپایل:</strong> هر ماژول تنها یک بار ارزیابی می‌شود؛ ایمپورت‌های بعدی محیط کامپایل‌شده را مجدداً استفاده می‌کنند.</li>
      </ul>

      <h2 id="syntax" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        روش‌ها و سینتکس ایمپورت
      </h2>

      <div className="space-y-6 my-4">
        <div>
          <h4 className="text-sm font-bold text-white">۱. ایمپورت مستقیم ماژول</h4>
          <p className="text-xs text-slate-400 mb-2">دسترسی به توابع از طریق فضای نام ماژول با نقطه:</p>
          <div dir="ltr" className="text-left">
            <CodeBlock
              language="persica"
              filename="app.persi"
              code={`import math;

print("Square root of 49.0:", math.sqrt(49.0));`}
              output={`Square root of 49.0: 7`}
            />
          </div>
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">۲. نام مستعار برای ماژول (`as`)</h4>
          <div dir="ltr" className="text-left">
            <CodeBlock
              language="persica"
              filename="app.persi"
              code={`import math as m;

print("Cosine of 0.0:", m.cos(0.0));`}
              output={`Cosine of 0.0: 1`}
            />
          </div>
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">۳. ایمپورت نمادهای خاص با `from`</h4>
          <p className="text-xs text-slate-400 mb-2">وارد کردن مستقیم شناسه‌ها به حوزه دید جاری:</p>
          <div dir="ltr" className="text-left">
            <CodeBlock
              language="persica"
              filename="app.persi"
              code={`from models import Point, User;
from math import sqrt as square_root;

Point p = Point(10, 20);
print("Direct access to sqrt:", square_root(25.0));`}
            />
          </div>
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">۴. ایمپورت تمام نمادها با ستاره (`*`)</h4>
          <div dir="ltr" className="text-left">
            <CodeBlock
              language="persica"
              filename="app.persi"
              code={`from utils.formatters import *;

// تمام توابع و کلاس‌های formatters اکنون مستقیم قابل دسترس هستند
print(format_currency(1500));`}
            />
          </div>
        </div>
      </div>

      <h2 id="folders" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ماژول‌های فولدری (`mod.persi` و `index.persi`)
      </h2>
      <p>
        اگر یک پکیج از چندین فایل داخلی تشکیل شده، آن‌ها را در یک پوشه قرار داده و یک فایل نقطه ورود به نام <code className="text-emerald-400">mod.persi</code> یا <code className="text-emerald-400">index.persi</code> برای آن ایجاد کنید:
      </p>

      <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60 font-mono text-xs text-slate-300 my-3" dir="ltr">
        <div>📁 src/</div>
        <div className="pl-4">├── 📁 geometry/</div>
        <div className="pl-8">├── 📄 circle.persi</div>
        <div className="pl-8">└── 📄 mod.persi &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// خروجی‌های پکیج هندسه</span></div>
        <div className="pl-4">└── 📄 main.persi</div>
      </div>

      <p className="text-xs text-slate-400">درون فایل <code className="text-emerald-300">src/main.persi</code>:</p>
      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="src/main.persi"
          code={`import geometry;
// کامپایلر این دستور را خودکار به geometry/mod.persi نگاشت می‌کند`}
        />
      </div>

      <h2 id="submodules" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        زیرماژول‌ها و ساختار نقطه‌ای
      </h2>
      <p>
        می‌توانید برای پیمایش پوشه‌های تو در تو از سینتکس نقطه‌ای پایتونی بهره ببرید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="app.persi"
          code={`from network.http.client import HttpClient;

HttpClient client = HttpClient();
print("Client initialized!");`}
        />
      </div>

      <h2 id="circular" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        محافظت در برابر ایمپورت‌های دایره‌ای (Circular Imports)
      </h2>
      <p>
        تحلیل‌گر گراف ماژول‌ها در پرسیکا وابستگی‌های جاری را پیگیری می‌کند. اگر فایل <code className="text-slate-300">A</code> فایل <code className="text-slate-300">B</code> را ایمپورت کند و <code className="text-slate-300">B</code> دوباره بخواهد <code className="text-slate-300">A</code> را وارد کند، کامپایلر بلافاصله بدون کرش یا سرریز حافظه، چرخه را گزارش می‌دهد.
      </p>
    </DocsLayout>
  );
}
