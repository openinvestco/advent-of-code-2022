import scala.io.Source

object Main 
{
    // Main Method 
    def main(args: Array[String]) =
    {
        val filename = "../test_input_scala.txt"
        for (line <- Source.fromFile(filename).getLines) {
            if (!line.isEmpty()) {
                val grouped = (" " + line).grouped(4).toList
                println(grouped)
            }
        }
    }
}