import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();

document.getElementById('train').onclick = function() {
    console.log(simulation.train());
};

const world = simulation.world();
const viewport = document.getElementById('viewport');
// ------------- ^------^
// | `document` - это глобальный объект, предоставляющий доступ и позволяющий модифицировать
// | текущую страницу (например, создавать и удалять элементы на ней).
//
// console.log(viewport)
const ctxt = viewport.getContext('2d');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const viewportScale = window.devicePixelRatio || 1;
// ------------------------------------------ ^^^^
// | Это похоже на `.unwrap_or(1)`
// |
// | Это значение определяет количество физических пикселей
// | в одном пикселе на холсте.
// |
// | Не HiDPI дисплеи обычно имеют плотность пикселей, равную 1.0.
// | Это означает, что рисование одного пикселя на холсте раскрасит
// | ровно один физический пиксель на экране.
// |
// | Мой дисплей имеет плотность пикселей, равную 2.0.
// | Это означает, что каждому пикселю, нарисованному на холсте,
// | будет соответствовать два физических пикселя, модифицированных браузером.
// ---

// Трюк, часть 1: мы увеличиваем *буфер* холста, чтобы он
// совпадал с плотностью пикселей экрана
viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

// Трюк, часть 2: мы уменьшаем *элемент* холста, поскольку
// браузер автоматически умножит его на плотность пикселей через мгновение.
//
// Это может показаться бесполезным, но суть заключается в том,
// что модификация размера элемента холста не влияет на
// размер его буфера, который *остается* увеличенным:
//
// ----------- < наша страница
// |         |
// |   ---   |
// |   | | < | < наш холст
// |   ---   |   (размер: viewport.style.width & viewport.style.height)
// |         |
// -----------
//
// За пределами страницы, в памяти браузера:
//
// ----- < буфер нашего холста
// |   | (размер: viewport.width & viewport.height)
// |   |
// -----
viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';


// const ctxt = viewport.getContext('2d');

// Автоматически масштабирует все операции на `viewportScale`, иначе
// нам пришлось бы `* viewportScale` все вручную
ctxt.scale(viewportScale, viewportScale);

// ---
// | Тип (точнее, прототип) нашего `ctxt`.
// v------------------ v
CanvasRenderingContext2D.prototype.drawTriangle =
    function (x, y, size, rotation) {
        this.beginPath();
        this.moveTo(
            x - Math.sin(rotation) * size * 1.5,
            y + Math.cos(rotation) * size * 1.5,
        );
        this.lineTo(
            x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        );
        this.lineTo(
            x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        );
        this.lineTo(
            x - Math.sin(rotation) * size * 1.5,
            y + Math.cos(rotation) * size * 1.5,
        );
        this.stroke();
        this.fillStyle = 'rgb(255, 255, 255)';
        this.fill();
    };

CanvasRenderingContext2D.prototype.drawCircle =
    function (x, y, radius) {
        this.beginPath();

        // ---
        // | Центр круга.
        // ----- v -v
        this.arc(x, y, radius, 0, 2.0 * Math.PI);
        // ------------------- ^ -^-----------^
        // | Начало и конец окружности, в радианах.
        // |
        // | Меняя эти параметры можно, например, нарисовать
        // | только половину круга.
        // ---

        this.fillStyle = 'rgb(0, 255, 128)';
        this.fill();
    };

function redraw() {
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

    simulation.step();

    for (const food of simulation.world().foods) {
        ctxt.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHeight,
            (0.01 / 2.0) * viewportWidth,
        );
    }

    for (const animal of simulation.world().animals) {
        ctxt.drawTriangle(
            animal.x * viewportWidth,
            animal.y * viewportHeight,
            0.01 * viewportWidth,
            animal.rotation,
        );
    }

    // requestAnimationFrame() планирует выполнение кода перед отрисовкой следующего кадра.
    //
    // Поскольку мы хотим, чтобы наша симуляция выполнялась вечно,
    // функцию необходимо зациклить
    requestAnimationFrame(redraw);
}



redraw();