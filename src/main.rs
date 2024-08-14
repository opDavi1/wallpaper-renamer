// This file is part of wprn by opDavi1, licensed under the MIT License.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::{env, process};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Not enough arguments!");
        wprn::show_usage();
        process::exit(1);
    } else if args[1] == "--help" {
        wprn::show_usage();
        process::exit(0);
    }

    let dir = Path::new(&args[1]);
    if let Err(e) = wprn::run(dir) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }   
}

