import Box from '@mui/material/Box';
import { DataGrid, GridToolbarContainer, GridToolbarFilterButton } from '@mui/x-data-grid';
import Link from 'next/link';

function CustomToolbar() {
  return (
    <GridToolbarContainer>
      <GridToolbarFilterButton />
    </GridToolbarContainer>
  );
}

const columns = [
  { 
    field: 'id', 
    headerName: 'ID', 
    width: 90,
  },
  {
    field: 'title',
    headerName: 'Title',
    minWidth: 400,
    sortable: false,
    renderCell: (params) => <Link href={`/leetcode/${params.row.id}`}>{params.row.title}</Link>
  },
  {
    field: 'difficulty',
    headerName: 'Difficulty',
    width: 100,
    sortable: false,
  },
  {
    field: 'tags',
    headerName: 'Tag',
    width: 150,
    sortable: false,
  },
];

export default function LCProblemTable({ problems }) {
  return (
    <Box sx={{ height: 2800, width: '100%', paddingTop: 3}}>
      <DataGrid
        rows={problems.map(p => p.frontmatter)}
        columns={columns}
        pageSize={20}
        rowsPerPageOptions={[20]}
        disableSelectionOnClick
        experimentalFeatures={{ newEditingApi: true }}
        components={{Toolbar: CustomToolbar}}
        disableColumnMenu
        sx={{
          boxShadow: 3,
          '& .MuiDataGrid-cell:hover': {
            color: 'primary.main',
          },
          "&.MuiDataGrid-root .MuiDataGrid-cell:focus-within": {
            outline: "none !important",
          },
        }}
      />
    </Box>
  );
}