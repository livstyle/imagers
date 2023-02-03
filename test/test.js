const rust = require('../');

// test('hello', () => {
//     expect(rust.hello()).toBe('hello node');
// });

test('img_resize', () => {
    const start = new Date().getTime();
    expect(rust.resize_by_path('images/rust.jpg', 'images/rust_thumb.jpg', 120, 120)).toBe('succeed');
    console.log(new Date().getTime() - start, ' ms');
});