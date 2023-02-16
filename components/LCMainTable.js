import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import { Link } from "@mui/material";

export default function LCProblemTable({ problems }) {
  return (
    <TableContainer>
      <Table aria-label="leetcode problem table">
        <TableHead>
          <TableRow>
            <TableCell>ID</TableCell>
            <TableCell>Title</TableCell>
            <TableCell>Difficulty</TableCell>
            <TableCell>Tags</TableCell>
          </TableRow>
        </TableHead>

        <TableBody>
          {problems.map(({ frontmatter }) => {
            const { id, title, diff, tags } = frontmatter;
            return (
              <TableRow key={id}>
                <TableCell>{id}</TableCell>
                <TableCell>
                  <Link href={`/leetcode/${id}`}>{title}</Link>
                </TableCell>
                <TableCell>{diff}</TableCell>
                <TableCell>{tags.join(", ")}</TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </TableContainer>
  );
}
