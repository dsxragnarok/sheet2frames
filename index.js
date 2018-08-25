const Jimp = require('jimp');
const path = require('path');

const arguments = process.argv.slice(2);
if (arguments < 2) {
   console.info('Usage:\n\tsheet2frames inputfile columns [rows]');
   return process.exit(1);
}

const range = (n) => Array.from({ length: n }, (value, key) => key);

const inputFile = arguments[0];
const columns = parseInt(arguments[1]);
const rows = arguments[2] ? parseInt(arguments[2], 10) : 1
const basename = path.basename(inputFile).slice(0, -1 * path.extname(inputFile).length);

Jimp.read(inputFile)
.then((sheet) => {
   const sheetWidth = sheet.getWidth();
   const sheetHeight = sheet.getHeight();
   const frameWidth = Math.floor(sheetWidth / columns);
   const frameHeight = Math.floor(sheetHeight / rows);

   range(rows).forEach((row) => range(columns).forEach((column) => {
      const frameIndex = (columns * row) + column;
      const x = column * frameWidth;
      const y = row * frameHeight;
      const filename = `${basename} (${frameIndex}).png`;

      console.info('processing\tframe\tx\ty\twidth\theight');
      console.info(`\t\t${frameIndex}\t${x}\t${y}\t${frameWidth}\t${frameHeight}`);

      new Jimp(frameWidth, frameHeight, (err, image) => {
         if (err) {
            console.error(err);
            return;
         }

         const frame = sheet.clone().crop(x, y, frameWidth, frameHeight);

         console.info(' -- saving', filename);
         image.composite(frame, 0, 0).write(filename);
      });
   }));
})
.catch((error) => console.error(error));
