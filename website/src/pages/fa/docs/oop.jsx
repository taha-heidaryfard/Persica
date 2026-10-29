import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';

export default function OopPageFa() {
  const meta = {
    title: 'استراکت‌ها و کلاس‌ها (OOP)',
    category: 'شیء‌گرایی و پایداری',
    description: 'مدل دوگانه شیء‌گرایی در پرسیکا: تفکیک ساختارهای مقداری داده خالص (struct) از کلاس‌های ارجاعی دارای متد و رفتار (class).',
  };

  const toc = [
    { title: 'فلسفه مدل شیء‌گرایی دوگانه', id: 'philosophy', level: 2 },
    { title: 'استراکت‌های داده خالص (struct)', id: 'structs', level: 2 },
    { title: 'کلاس‌های شیء‌گرا (class)', id: 'classes', level: 2 },
    { title: 'متدها و کلیدواژه this', id: 'methods', level: 2 },
    { title: 'تفاوت معنایی ارجاع در برابر مقدار', id: 'semantics', level: 2 },
    { title: 'مجموعه‌ها و آرایه‌های اشیاء', id: 'collections', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="philosophy" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        فلسفه مدل شیء‌گرایی دوگانه
      </h2>
      <p>
        پرسیکا با ارائه دو ساختار مجزا و با هدف مشخص، یکی از چالش‌های همیشگی در طراحی زبان‌های برنامه‌نویسی را حل کرده است:
      </p>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 my-4 text-xs">
        <div className="p-4 rounded-xl border border-blue-500/30 bg-blue-500/10 space-y-1.5">
          <h4 className="font-bold text-blue-300 text-sm">struct (داده خالص و سبک)</h4>
          <p className="text-slate-300">برای نگهداری مدل‌های داده‌ای سبک نظیر مختصات، ابعاد، رنگ‌ها و نقاط.</p>
          <ul className="list-disc pr-4 space-y-0.5 text-slate-400">
            <li><strong>معنای مقداری (Value Semantics):</strong> هنگام انتساب کپی می‌شود.</li>
            <li><strong>بدون متد:</strong> صرفاً نگهدارنده فیلدهای داده‌ای.</li>
            <li><strong>مقایسه عمیق:</strong> برابری ساختاری تمام فیلدها.</li>
          </ul>
        </div>

        <div className="p-4 rounded-xl border border-emerald-500/30 bg-emerald-500/10 space-y-1.5">
          <h4 className="font-bold text-emerald-300 text-sm">class (وضعیت و رفتار پویا)</h4>
          <p className="text-slate-300">برای موجودیت‌های فعال نظیر حساب بانکی، کلاینت پایگاه‌داده و اتصالات شبکه.</p>
          <ul className="list-disc pr-4 space-y-0.5 text-slate-400">
            <li><strong>معنای ارجاعی (Reference Semantics):</strong> انتساب اشاره‌گر مشترک در حافظه Heap.</li>
            <li><strong>دارای متد:</strong> دسترسی مستقیم به وضعیت داخلی از طریق <code className="text-emerald-300">this</code>.</li>
            <li><strong>هویت مشترک:</strong> تغییر در یک متغیر در تمامی دارندگان ارجاع منعکس می‌شود.</li>
          </ul>
        </div>
      </div>

      <h2 id="structs" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        استراکت‌های داده خالص (`struct`)
      </h2>
      <p>
        برای تعریف استراکت نام فیلدها و نوع آن‌ها را می‌نویسیم. ساخت نمونه با فراخوانی نام استراکت شبیه تابع انجام می‌گیرد:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="struct_demo.persi"
          code={`struct Point {
    int x;
    int y;
}

Point p1 = Point(10, 20);
print("Point p1:", p1.x, p1.y);

// کپی مقداری: تغییر در p2 روی p1 تأثیری ندارد
Point p2 = p1;
p2.x = 99;

print("p1.x after p2 change:", p1.x);
print("p2.x:", p2.x);`}
          output={`Point p1: 10 20
p1.x after p2 change: 10
p2.x: 99`}
        />
      </div>

      <h2 id="classes" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        کلاس‌های شیء‌گرا (`class`)
      </h2>
      <p>
        کلاس‌ها فیلدها را با مقادیر پیش‌فرض اختیاری و متدها درون یک پکیج کپسوله‌سازی می‌کنند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="class_demo.persi"
          code={`class StopWatch {
    int elapsed_seconds = 0;
    bool is_running = false;

    void start() {
        this.is_running = true;
    }

    void tick(int seconds) {
        if this.is_running {
            this.elapsed_seconds += seconds;
        }
    }

    int get_elapsed() {
        return this.elapsed_seconds;
    }
}

StopWatch timer = StopWatch();
timer.start();
timer.tick(15);
timer.tick(30);
print("Elapsed time:", timer.get_elapsed(), "seconds");`}
          output={`Elapsed time: 45 seconds`}
        />
      </div>

      <h2 id="methods" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        متدها و کلیدواژه `this`
      </h2>
      <p>
        درون هر متد، کلیدواژه <code className="text-emerald-400 font-bold">this</code> به نمونه فعال فعلی اشاره دارد و می‌تواند متدهای همجوار را صدا بزند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="bank_account.persi"
          code={`class Account {
    string owner;
    int balance = 0;

    void deposit(int amount) {
        this.balance += amount;
        this.log_audit("Deposited " + to_string(amount));
    }

    void log_audit(string event) {
        print("[AUDIT]", this.owner, "-", event, "| Balance:", this.balance);
    }
}

Account acc = Account();
acc.owner = "محمد طاها";
acc.deposit(500);
acc.deposit(250);`}
          output={`[AUDIT] محمد طاها - Deposited 500 | Balance: 500
[AUDIT] محمد طاها - Deposited 250 | Balance: 750`}
        />
      </div>

      <h2 id="semantics" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        تفاوت معنایی ارجاع در برابر مقدار
      </h2>
      <p>
        هنگامی که یک شیء <code className="text-emerald-300 font-semibold">class</code> را به متغیر دیگری منتسب می‌کنید، هر دو به یک وهله در حافظه هیپ (مدیریت‌شده با شمارش ارجاع امن در راست) اشاره می‌کنند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="references.persi"
          code={`class SharedState {
    int counter = 0;
}

SharedState ref1 = SharedState();
SharedState ref2 = ref1; // ارجاع مشترک

ref2.counter = 42;

// هر دو به یک آبجکت اشاره دارند، بنابراین ref1 نیز ۴۲ را می‌بیند
print("ref1.counter:", ref1.counter);`}
          output={`ref1.counter: 42`}
        />
      </div>

      <h2 id="collections" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        مجموعه‌ها و آرایه‌های اشیاء
      </h2>
      <p>
        هم استراکت‌ها و هم کلاس‌ها می‌توانند درون آرایه‌ها و مپ‌ها قرار گیرند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="object_lists.persi"
          code={`struct Task {
    int id;
    string title;
    bool done;
}

Task[] todo_list = [
    Task(1, "Install Persica", true),
    Task(2, "Learn OOP syntax", true),
    Task(3, "Build awesome app", false)
];

for task in todo_list {
    string status = task.done ? "[DONE]" : "[PENDING]";
    print(status, task.title);
}`}
          output={`[DONE] Install Persica
[DONE] Learn OOP syntax
[PENDING] Build awesome app`}
        />
      </div>
    </DocsLayout>
  );
}
