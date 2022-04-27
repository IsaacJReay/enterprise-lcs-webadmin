import React, { useState, useEffect } from "react";
import {
  Layout,
  Col,
  Row,
  Button,
  Form,
  Table,
  Select,
  Tag,
  Popconfirm,
  message,
  Collapse,
  Checkbox,
  Space,
} from "antd";
import { FiEdit } from "react-icons/fi";
import DNSRename from "./rename";
import { Link, useParams } from "react-router-dom";
import AddRecord from "./adding-record";
import { PlusOutlined, CaretRightOutlined } from "@ant-design/icons";
import { IoIosHelpCircle } from "react-icons/io";
import axios from "axios";

const { Content } = Layout;
const { Option } = Select;
const { Panel } = Collapse;

const DNSManagement = ({ match }) => {
  const [visible, setVisible] = useState(false);
  const [records, setRecords] = useState(false);
  const [domainStatus, setDomainStatus] = useState({});
  const [, setLoading] = useState(false);
  const [items, setItems] = useState({});
  const [subdomain, setSubdomain] = useState([]);
  const [form] = Form.useForm();

  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  let { slug } = useParams();
  const zone = match.params.zones;

  // ----------get data -------------

  async function fetchData() {
    await axios
      .get(`${baseUrl}/settings/dns/status/${zone}/${slug}`, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        setLoading(true);
        setItems(res.data);
        setSubdomain(res.data.zone_record);
        setDomainStatus(res.data.status);
        const { status } = res.data;
        form.setFieldsValue({
          status: status,
        });
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }

  useEffect(() => {
    fetchData();
  }, []);

  //  --------delete record ----------

  const handleDelete = async (name) => {
    await axios
      .delete(`${baseUrl}/settings/dns/delete/${zone}/${slug}/${name}`, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          fetchData();
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        console.log(err);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error("Operation Failed! ");
      });
  };

  // ---------- hosting domain ----------
  const handleApply = async (data) => {
    const inputData = {
      domain_name: slug,
      status: data.status,
      zone_record: null,
    };
    console.log(inputData);
    await axios
      .post(`${baseUrl}/settings/dns/new/${zone}`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          fetchData();
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        console.log(err);
        message.error("Operation Failed! ");
      });
  };

  const showModalRename = () => {
    setVisible(true);
  };

  const handleCancel = () => {
    setVisible(false);
    setRecords(false);
  };

  const handleOk = () => {
    setVisible(false);
    setRecords(false);
  };

  const showAddRecords = () => {
    setRecords(true);
  };

  const columns = [
    {
      title: "Sub Domain",
      dataIndex: "subdomain_name",
      key: "subdomain_name",
      editable: true,
      width: "40%",
    },
    {
      title: "Address",
      dataIndex: "address",
      key: "address",
      editable: true,
      width: "20%",
    },
    {
      title: "Type",
      dataIndex: "dns_type",
      key: "dns_type",
      width: "20%",
      editable: true,
      render: (dns_type) => {
        return (
          <React.Fragment>
            <Select
              disabled
              defaultValue={dns_type}
              size="large"
              className="select-option2"
            >
              <Option value="A">A</Option>
              <Option value="MX 10">MX 10</Option>
              <Option value="CNAME">CNAME</Option>
            </Select>
          </React.Fragment>
        );
      },
    },
    {
      title: "Actions",
      dataIndex: "id",
      width: "20%",
      render: (id, data) => {
        const name = data.subdomain_name;
        return (
          <React.Fragment>
            <Popconfirm
              placement="top"
              title={
                <span>
                  Are you sure to delete <b>{name} </b>?
                </span>
              }
              okText="Yes"
              cancelText="No"
              onConfirm={() => handleDelete(name)}
            >
              <Tag color="error" style={{ cursor: "pointer" }}>
                Delete
              </Tag>
            </Popconfirm>
          </React.Fragment>
        );
      },
    },
  ];

  const HostingDomain = () => {
    return (
      <React.Fragment>
        <div className="card3">
          <div className="container">
            <div className="container-header">
              <h1>Domain Setting</h1>
            </div>
            <hr />

            <Collapse
              bordered={false}
              defaultActiveKey={["1"]}
              expandIcon={({ isActive }) => (
                <CaretRightOutlined rotate={isActive ? 90 : 0} />
              )}
            >
              <Panel header="Hosting Domain!" key="1">
                <Form onFinish={handleApply} form={form}>
                  <Form.Item name="status" valuePropName="checked">
                    <Checkbox style={{ color: "red ", paddingLeft: "20px" }}>
                      Hosting Domain!
                    </Checkbox>
                  </Form.Item>
                  <Form.Item>
                    <Button
                      type="primary"
                      className="button-apply2"
                      htmlType="submit"
                      size="large"
                    >
                      APPLY
                    </Button>
                  </Form.Item>
                </Form>
              </Panel>
            </Collapse>
          </div>
        </div>
      </React.Fragment>
    );
  };

  return (
    <React.Fragment>
      <Content>
        <DNSRename
          visible={visible}
          handleCancel={handleCancel}
          handleOk={handleOk}
          fetchData={fetchData}
          zone={zone}
        />
        <AddRecord
          records={records}
          handleCancel={handleCancel}
          handleOk={handleOk}
          fetchData={fetchData}
          zone={zone}
          domainStatus={domainStatus}
        />
        <Row gutter={12}>
          <Col span={16}>
            <Row gutter={[12, 12]}>
              <Col span={24}>
                <div>
                  <HostingDomain />
                </div>
              </Col>
              <Col>
                <Form>
                  <div className="card3">
                    <div className="container">
                      <div className="container-header">
                        <h1>DNS SETTING</h1>
                      </div>
                      <div className="dns-desc-container">
                        <Form.Item label="Domain Name">
                          <Row gutter={[6, 0]}>
                            <Col>
                              <p className="domain_name">{items.domain_name}</p>
                            </Col>
                            <Col>
                              <FiEdit onClick={showModalRename} />
                            </Col>
                          </Row>
                        </Form.Item>
                      </div>

                      <div className="dns-desc-container">
                        <Form.Item>
                          <Button
                            type="primary"
                            className="button-update"
                            onClick={showAddRecords}
                          >
                            <PlusOutlined />
                            Add Record
                          </Button>
                        </Form.Item>
                        <Table
                          columns={columns}
                          dataSource={subdomain}
                          pagination={false}
                          scroll={{ y: 200 }}
                        />
                      </div>

                      <div className="container-buttons">
                        <Form.Item>
                          <Link to="/dns">
                            <Button
                              type="primary"
                              htmlType="button"
                              className="button-apply2"
                              size="large"
                            >
                              Back
                            </Button>
                          </Link>
                        </Form.Item>
                      </div>
                    </div>
                  </div>
                </Form>
              </Col>
            </Row>
          </Col>
          <Col span={8}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <Space>
                    <h1>HELPS</h1>
                    <IoIosHelpCircle className="icon-help" />
                  </Space>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default DNSManagement;
