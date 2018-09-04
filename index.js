const Jimp = require('jimp');
const path = require('path');

const arguments = process.argv.slice(2);
if (arguments < 2) {
   console.info('Usage:\n\tsheet2frames inputfile [outputDir] columns [rows]');
   return process.exit(1);
}
const range = (n) => Array.from({ length: n }, (value, key) => key);

const inputFile = arguments[0];
const outputDir = !Number.isInteger(parseInt(arguments[1]), 10) ? arguments[1] : '';
const columns = arguments.length > 2 ? parseInt(arguments[2]) : parseInt(arguments[1], 10);
const rows = arguments.length > 3 ? parseInt(arguments[3], 10) :
            arguments.length === 3 && outputDir.length === 0 ? parseInt(arguments[2], 10) :
            1;
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
      const outputPath = path.join(outputDir, filename);

      console.info('processing\tframe\tx\ty\twidth\theight');
      console.info(`\t\t${frameIndex}\t${x}\t${y}\t${frameWidth}\t${frameHeight}`);

      new Jimp(frameWidth, frameHeight, (err, image) => {
         if (err) {
            console.error(err);
            return;
         }

         const frame = sheet.clone().crop(x, y, frameWidth, frameHeight);

         console.info(' -- saving', outputPath);
         image.composite(frame, 0, 0).write(outputPath);
      });
   }));
})
.catch((error) => console.error(error));
